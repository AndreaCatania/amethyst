
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
use nalgebra::Transform3;
use nalgebra::Isometry3;

pub(crate) struct VecConversor;

impl VecConversor{
    pub fn to_physics(v: &Vector3<Float>) -> Vector3<f32> {
        Vector3::new(v.x.as_f32(), v.y.as_f32(), v.z.as_f32())
    }

    pub fn from_physics(v: &Vector3<f32>) -> Vector3<Float> {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub(crate) struct QuatConversor;

impl QuatConversor{
    pub fn to_physics(r: &Quaternion<Float>) -> Quaternion<f32> {

        Quaternion::from(Vector4::new(r.i.as_f32(), r.j.as_f32(), r.k.as_f32(), r.w.as_f32()))
    }

    pub fn from_physics(r: &Quaternion<f32>) -> Quaternion<Float> {

        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }
}

pub(crate) struct TransfConversor;

impl TransfConversor {
    pub fn to_physics(t: &Transform) -> Isometry3<f32> {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(t.translation())),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(t.rotation())) )

    }

    pub fn from_physics(t: &Isometry3<f32>) -> Transform {
        Transform::new(t.translation, t.rotation, Vector3::new(1.0, 1.0, 1.0))
    }
}
