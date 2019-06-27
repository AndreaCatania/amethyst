

use amethyst_core::{
    components::Transform,
    Float,
    math::{
        Vector3,
        Translation3,
        Quaternion,
        Vector4,
        UnitQuaternion,
    },
};

use nalgebra::{
    Transform3,
    Isometry3,
    RealField,
};


pub(crate) struct VecConversor;

impl VecConversor{
    pub fn to_physics<N>(v: &Vector3<Float>) -> Vector3<N>
        where N: RealField,
              Float: std::convert::Into<N>
    {
            Vector3::new(
               v.x.into(),
               v.y.into(),
               v.z.into())
    }

    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<Float>
        where N: RealField,
              Float: std::convert::From<N>
    {

        Vector3::new(
               Float::from(v.x),
               Float::from(v.y),
               Float::from(v.z))

    }
}

pub(crate) struct QuatConversor;

impl QuatConversor{
    pub fn to_physics<N>(r: &Quaternion<Float>) -> Quaternion<N>
        where N: RealField,
              Float: std::convert::Into<N>
    {

        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    pub fn from_physics<N>(r: &Quaternion<N>) -> Quaternion<Float>
        where N: RealField,
              Float: std::convert::From<N>
    {

        Quaternion::from(
                Vector4::new(
                    Float::from(r.i),
                    Float::from(r.j),
                    Float::from(r.k),
                    Float::from(r.w)))
    }
}

pub(crate) struct TransfConversor;

impl TransfConversor {
    pub fn to_physics<N>(t: &Transform) -> Isometry3<N>
        where N: RealField,
              Float: std::convert::Into<N>
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(t.translation())),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(t.rotation())) )

    }

    pub fn from_physics<N>(t: &Isometry3<N>) -> Transform
        where N: RealField,
              Float: std::convert::From<N>,
              N: alga::general::SubsetOf<Float>
    {
        Transform::new(t.translation, t.rotation, Vector3::x() + Vector3::y() + Vector3::z())
    }
}

pub mod body_mode_conversor{
    use amethyst_phythyst::servers::BodyMode;

    use nphysics3d::object::{
        BodyStatus as NpBodyStatus,
    };

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
