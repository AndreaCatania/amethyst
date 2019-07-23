use amethyst_core::{
    components::Transform,
    math::{Quaternion, Translation3, UnitQuaternion, Vector3, Vector4},
};

use nalgebra::{Isometry3, RealField, Transform3};

pub(crate) struct VecConversor;

// TODO this may be removed. Check this please

impl VecConversor {
    pub fn to_physics<N>(v: &Vector3<f32>) -> Vector3<N>
    where
        N: RealField + From<f32>,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }

    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<f32>
    where
        N: RealField,
        f32: From<N>,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub(crate) struct QuatConversor;

impl QuatConversor {
    pub fn to_physics<N>(r: &Quaternion<f32>) -> Quaternion<N>
    where
        N: RealField,
        f32: std::convert::Into<N>,
    {
        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    pub fn from_physics<N>(r: &Quaternion<N>) -> Quaternion<f32>
    where
        N: RealField,
        f32: std::convert::From<N>,
    {
        Quaternion::from(Vector4::new(
            f32::from(r.i),
            f32::from(r.j),
            f32::from(r.k),
            f32::from(r.w),
        ))
    }
}

pub(crate) struct TransfConversor;

impl TransfConversor {
    pub fn to_physics<N>(t: &Isometry3<f32>) -> Isometry3<N>
    where
        N: RealField + std::convert::From<f32>,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(&t.rotation)),
        )
    }

    pub fn from_physics<N>(t: &Isometry3<N>) -> Isometry3<f32>
    where
        N: RealField,
        f32: std::convert::From<N>,
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
