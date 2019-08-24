use std::sync::{Arc, RwLock};

use amethyst_core::ecs::{Component, DenseVecStorage, FlaggedStorage};

macro_rules! define_opaque_object {
    ($what:ident, $gc_name:ident) => {
        /// This is an opaque ID that is created by a physics server.
        /// Create this Opaque ID manually is not safe, for this reason is marked as so.
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $what {
            U32(u32),
            U64(u64),

            U32U32(u32, u32),
            U64U64(u64, u64),

            UsizeU32(usize, u32),
            UsizeU64(usize, u64),
        }

        impl $what {
            pub unsafe fn new_u32(a: u32) -> Self {
                $what::U32(a)
            }

            pub unsafe fn new_u64(a: u64) -> Self {
                $what::U64(a)
            }

            pub unsafe fn new_u32u32(a: u32, b: u32) -> Self {
                $what::U32U32(a, b)
            }

            pub unsafe fn new_u64u64(a: u64, b: u64) -> Self {
                $what::U64U64(a, b)
            }

            pub unsafe fn new_usizeu32(a: usize, b: u32) -> Self {
                $what::UsizeU32(a, b)
            }

            pub unsafe fn new_usizeu64(a: usize, b: u64) -> Self {
                $what::UsizeU64(a, b)
            }
        }

        impl PhysicsTag for $what {
            fn request_resource_removal(&mut self, gc: &mut PhysicsGarbageCollector) {
                gc.$gc_name.push(*self);
            }
        }
    };
}

define_opaque_object!(PhysicsRigidBodyTag, bodies);
define_opaque_object!(PhysicsAreaTag, areas);
define_opaque_object!(PhysicsShapeTag, shapes);
define_opaque_object!(PhysicsJointTag, joints);

/// This is used only to perform the setup of these storages.
///
/// The setup happens in the `PhysicsBundle`.
pub(crate) type PhysicsSetupStorages<'a> = (
    amethyst_core::ecs::ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
    amethyst_core::ecs::ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
    amethyst_core::ecs::ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
    amethyst_core::ecs::ReadStorage<'a, PhysicsHandle<PhysicsJointTag>>,
);

/// This trait must be implemented for each structure that want to use the PhysicsHandle.
pub trait PhysicsTag: Copy + std::fmt::Debug + Sync + Send + Sized + 'static {
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
            "PhysicsHandle{{\n   tag = {:?}\n   owner = {}\n   weak = {}\n}};",
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
/// When this container is dropped it requests the dropping of the resource to the GC.
///
/// Thanks to the container, we can have many `PhysicsHandle` which all points to the same data.
/// This mechanism, make sure to avoid too much useless copy and give a simple way to notify the GC
/// only once, even if there are more handles of a resource.
///
/// The function `request_resource_removal`, that notify the GC is implemented per `PhysicsTag` to
/// allow custom signaling operations depending on the tag type.
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
    pub bodies: Vec<PhysicsRigidBodyTag>,
    pub areas: Vec<PhysicsAreaTag>,
    pub shapes: Vec<PhysicsShapeTag>,
    pub joints: Vec<PhysicsJointTag>,
}

impl Default for PhysicsGarbageCollector {
    fn default() -> Self {
        PhysicsGarbageCollector {
            bodies: Vec::new(),
            areas: Vec::new(),
            shapes: Vec::new(),
            joints: Vec::new(),
        }
    }
}
