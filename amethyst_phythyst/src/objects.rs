use amethyst_core::ecs::{Component, DenseVecStorage};

macro_rules! define_opaque_object{
    ($what:ident, $doc_name:ident) => {
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

        impl Component for $what {
            type Storage = DenseVecStorage<$what>;
        }

        impl std::ops::Deref for $what {
            type Target = std::num::NonZeroUsize;
            fn deref(&self) -> &std::num::NonZeroUsize {
                &self.0
            }
        }
    }
}

define_opaque_object!(PhysicsWorldTag, World);
define_opaque_object!(PhysicsBodyTag, Rigid_Body);
define_opaque_object!(PhysicsAreaTag, Area);
define_opaque_object!(PhysicsShapeTag, Shape);
