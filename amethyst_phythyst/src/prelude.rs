//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    objects::{
        PhysicsAreaTag, PhysicsBodyTag, PhysicsGarbageCollector, PhysicsHandle, PhysicsShapeTag,
        PhysicsTag,
    },
    servers::{
        AreaDesc, AreaPhysicsServerTrait, PhysicsWorld, RBodyPhysicsServerTrait, BodyMode, RigidBodyDesc,
        ShapeDesc, ShapePhysicsServerTrait, WorldPhysicsServerTrait,
    },
};
