use amethyst_core::{
    components::Transform,
    math::{Quaternion, Translation3, UnitQuaternion, Vector3, Vector4},
};
use amethyst_phythyst::objects::{
    PhysicsAreaTag, PhysicsJointTag, PhysicsRigidBodyTag, PhysicsShapeTag,
};
use nalgebra::{Isometry3, Transform3};

use crate::storage::StoreKey;

pub(crate) struct VecConversor;

// TODO this may be removed. Check this please

impl VecConversor {
    pub fn to_physics<N>(v: &Vector3<f32>) -> Vector3<N>
    where
        N: amethyst_phythyst::PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }

    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<f32>
    where
        N: amethyst_phythyst::PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub(crate) struct QuatConversor;

impl QuatConversor {
    pub fn to_physics<N>(r: &Quaternion<f32>) -> Quaternion<N>
    where
        N: amethyst_phythyst::PtReal,
    {
        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    pub fn from_physics<N>(r: &Quaternion<N>) -> Quaternion<f32>
    where
        N: amethyst_phythyst::PtReal,
    {
        Quaternion::from(Vector4::new(
            N::into(r.i),
            N::into(r.j),
            N::into(r.k),
            N::into(r.w),
        ))
    }
}

pub(crate) struct TransfConversor;

impl TransfConversor {
    pub fn to_physics<N>(t: &Isometry3<f32>) -> Isometry3<N>
    where
        N: amethyst_phythyst::PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(&t.rotation)),
        )
    }

    pub fn from_physics<N>(t: &Isometry3<N>) -> Isometry3<f32>
    where
        N: amethyst_phythyst::PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::from_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::from_physics(&t.rotation)),
        )
    }
}

pub mod body_mode_conversor {
    use amethyst_phythyst::servers::BodyMode;

    use nphysics3d::object::BodyStatus as NpBodyStatus;

    pub fn to_physics(m: BodyMode) -> NpBodyStatus {
        match m {
            BodyMode::Disabled => NpBodyStatus::Disabled,
            BodyMode::Static => NpBodyStatus::Static,
            BodyMode::Dynamic => NpBodyStatus::Dynamic,
            BodyMode::Kinematic => NpBodyStatus::Kinematic,
        }
    }

    pub fn from_physics(s: NpBodyStatus) -> BodyMode {
        match s {
            NpBodyStatus::Disabled => BodyMode::Disabled,
            NpBodyStatus::Static => BodyMode::Static,
            NpBodyStatus::Dynamic => BodyMode::Dynamic,
            NpBodyStatus::Kinematic => BodyMode::Kinematic,
        }
    }
}

macro_rules! opaque_conversors {
    ($t:ident, $to:ident, $from:ident, $test_mod:ident) => {
        pub fn $to(tag: $t) -> StoreKey {
            match tag {
                $t::UsizeU64(a, b) => StoreKey::new(a, b),
                _ => {
                    // If happens, something is strange
                    panic!();
                }
            }
        }

        pub fn $from(key: StoreKey) -> $t {
            unsafe { key.map(|index, generation| $t::new_usizeu64(index, generation)) }
        }

        #[cfg(test)]
        mod $test_mod {
            use crate::conversors::*;

            #[test]
            fn test() {
                let tag = unsafe { $t::new_usizeu64(1, 10) };
                let key = $to(tag);
                assert_eq!(tag, $from(key));
            }
        }
    };
}

opaque_conversors!(
    PhysicsRigidBodyTag,
    rigid_tag_to_store_key,
    store_key_to_rigid_tag,
    test_conversors_physics_rigid_body_tag
);
opaque_conversors!(
    PhysicsAreaTag,
    area_tag_to_store_key,
    store_key_to_area_tag,
    test_conversors_physics_area_tag
);
opaque_conversors!(
    PhysicsShapeTag,
    shape_tag_to_store_key,
    store_key_to_shape_tag,
    test_conversors_physics_shape_tag
);
opaque_conversors!(
    PhysicsJointTag,
    joint_tag_to_store_key,
    store_key_to_joint_tag,
    test_conversors_physics_joint_tag
);
