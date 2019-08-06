use std::sync::{Arc, RwLock};

use amethyst_core::ecs::{Component, DenseVecStorage, FlaggedStorage};

macro_rules! define_opaque_object {
    ($what:ident, $doc_name:ident, $gc_name:ident) => {
        /// $what is the opaque ID that identify a `$doc_name` in the physics server
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $what(pub std::num::NonZeroUsize);

        /// Panic if called
        impl Default for $what {
            fn default() -> Self {
                panic!();
                $what(std::num::NonZeroUsize::new(1).unwrap())
            }
        }

        impl std::ops::Deref for $what {
            type Target = std::num::NonZeroUsize;
            fn deref(&self) -> &std::num::NonZeroUsize {
                &self.0
            }
        }

        impl PhysicsTag for $what {
            fn request_resource_removal(&mut self, gc: &mut PhysicsGarbageCollector) {
                gc.$gc_name.push(*self);
            }
        }

        impl std::fmt::Display for $what {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "$what({})", self.0)
            }
        }
    };
}

define_opaque_object!(PhysicsWorldTag, World, worlds);
define_opaque_object!(PhysicsBodyTag, Rigid_Body, bodies);
define_opaque_object!(PhysicsAreaTag, Area, areas);
define_opaque_object!(PhysicsShapeTag, Shape, shapes);

/// This trait must be implemented for each structure that want to use the PhysicsHandle.
pub trait PhysicsTag: Copy + std::fmt::Display + Sync + Send + Sized + 'static {
    fn request_resource_removal(&mut self, gc: &mut PhysicsGarbageCollector);
}

/// The physics handle is used to track the physics resource lifetime.
/// Indeed you don't have to care about dropping resources (life a RigidBody or a Shape) because
/// they are automatically cleaned out once all PhysicsHandle to that object are dropped.
///
/// Worth to mention that you can store these handle anywhere, and the GC will always take care of
/// its dropping.
///
/// If you need a copy of this resource you can simply use the function `clone()`.
///
/// All Physics Servers APIs want to deal directly with the PhysicsTag.
/// Use the method `get()` to retrieve it.
/// Keep in mind that it's lifetime is not tracked by the GC, thus is not a replacement of the PhysicsHandle.
pub struct PhysicsHandle<T: PhysicsTag> {
    tag_container: Arc<PhysicsTagContainer<T>>,
}

impl<T: PhysicsTag> PhysicsHandle<T> {
    pub fn new(tag: T, garbage_collector: Arc<RwLock<PhysicsGarbageCollector>>) -> Self {
        PhysicsHandle {
            tag_container: Arc::new(PhysicsTagContainer {
                tag,
                garbage_collector,
            }),
        }
    }

    /// Returns the PhysicsTag
    /// Keep in mind that this doesn't alter the resource lifetime in anyway.
    pub fn get(&self) -> T {
        self.tag_container.tag
    }
}

impl<T: PhysicsTag> std::fmt::Debug for PhysicsHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PhysicsHandle{{\n   tag = {}\n   owner = {}\n   weak = {}\n}};",
            self.get(),
            Arc::strong_count(&self.tag_container),
            Arc::weak_count(&self.tag_container)
        )
    }
}

impl<T: PhysicsTag> Clone for PhysicsHandle<T> {
    fn clone(&self) -> Self {
        PhysicsHandle {
            tag_container: self.tag_container.clone(),
        }
    }
}

impl<T: PhysicsTag> Component for PhysicsHandle<T> {
    type Storage = FlaggedStorage<PhysicsHandle<T>, DenseVecStorage<PhysicsHandle<T>>>;
}

/// This container holds both the Tag and the garbage collector.
/// When this container is dropped it request the dropping of the resource to the GC.
///
/// The reason why the task to signal the resource dropping got assigned to the container
/// is because in this way is possible to pass just the ID of the resource to the server APIs
/// avoiding so useless copy of the GC pointer.
///
/// The code that execute the signaling operation is implemented per PhysicsTag to allow custom
/// signaling depending on the tag.
struct PhysicsTagContainer<T: PhysicsTag> {
    tag: T,
    garbage_collector: Arc<RwLock<PhysicsGarbageCollector>>,
}

impl<T: PhysicsTag> std::ops::Drop for PhysicsTagContainer<T> {
    fn drop(&mut self) {
        let mut gc = self.garbage_collector.write().unwrap();
        self.tag.request_resource_removal(&mut gc);
    }
}

/// This garbage collector is used to store all the PhysicsTags to whom its associated handle get dropped.
///
/// The main benefit to use the Garbage Collector is that each PhysicsServer can implement its own destructor
/// pipeline.
/// Another benefit is that the user can store the PhysicsHandles even as resource or as prefer.
///
/// The alternative implementation was use a flagged storage.
/// Using a FlaggedStorage would have been not only less powerful (since the objects are not tracked
/// if stored elsewhere), but even more complicate.
/// Indeed the FlaggedStorage has an handy Event system, which returns only the storage Index of the
/// associated event.
/// What this mean in practice is that you don't have access to PhysicsTag ID because the Index get
/// removed and the only way would have been re implement a new storage with the capability to return
/// PhysicsTag on component drop.
/// Also the destruction pipeline is dictated by phythyst to each physics backend.
///
/// Considering the above the GC seems a better way.
pub struct PhysicsGarbageCollector {
    pub worlds: Vec<PhysicsWorldTag>,
    pub bodies: Vec<PhysicsBodyTag>,
    pub areas: Vec<PhysicsAreaTag>,
    pub shapes: Vec<PhysicsShapeTag>,
}

impl Default for PhysicsGarbageCollector {
    fn default() -> Self {
        PhysicsGarbageCollector {
            worlds: Vec::new(),
            bodies: Vec::new(),
            areas: Vec::new(),
            shapes: Vec::new(),
        }
    }
}
