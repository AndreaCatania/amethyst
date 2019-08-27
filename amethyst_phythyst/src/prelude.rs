//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    objects::{
        PhysicsAreaTag, PhysicsGarbageCollector, PhysicsHandle, PhysicsJointTag,
        PhysicsRigidBodyTag, PhysicsShapeTag, PhysicsTag,
    },
    servers::{
        AreaDesc, AreaPhysicsServerTrait, BodyMode, JointDesc, JointPhysicsServerTrait,
        PhysicsWorld, RBodyPhysicsServerTrait, RigidBodyDesc, ShapeDesc, ShapePhysicsServerTrait,
        WorldPhysicsServerTrait,
    },
    PhysicsTime,
};
