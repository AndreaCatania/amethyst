
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
    pub fn to_physics<N: RealField>(v: &Vector3<Float>) -> Vector3<N>
        where N: std::convert::From<amethyst_core::Float>
    {
            Vector3::new(
               v.x.into(),
               v.y.into(),
               v.z.into())
    }

    pub fn from_physics<N: RealField>(v: &Vector3<N>) -> Vector3<Float>
        where amethyst_core::Float: std::convert::From<N>
    {

        Vector3::new(
               v.x.into(),
               v.y.into(),
               v.z.into())

    }
}

pub(crate) struct QuatConversor;

impl QuatConversor{
    pub fn to_physics<N: RealField + std::convert::From<Float>>(r: &Quaternion<Float>) -> Quaternion<N> {

        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    pub fn from_physics<N: RealField, Float: RealField + std::convert::From<N>>(r: &Quaternion<N>) -> Quaternion<Float> {

        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }
}

pub(crate) struct TransfConversor;

impl TransfConversor {
    pub fn to_physics<N: RealField + std::convert::From<Float> + alga::general::SubsetOf<Float>>(t: &Transform) -> Isometry3<N> {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(t.translation())),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(t.rotation())) )

    }

    pub fn from_physics<N: RealField + std::convert::From<amethyst_core::Float> + alga::general::SubsetOf<Float>>(t: &Isometry3<N>) -> Transform {
        Transform::new(t.translation, t.rotation, Vector3::x() + Vector3::y() + Vector3::z())
    }
}
