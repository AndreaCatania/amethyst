use amethyst_core::{
    components::Transform,
    math::{Isometry3, Quaternion, Translation3, UnitQuaternion, Vector3, Vector4},
};

use crate::PtReal;

pub struct VecConversor;

impl VecConversor {
    pub fn to_physics<N>(v: &Vector3<f32>) -> Vector3<N>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }

    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<f32>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub struct QuatConversor;

impl QuatConversor {
    pub fn to_physics<N>(r: &Quaternion<f32>) -> Quaternion<N>
    where
        N: PtReal,
    {
        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    pub fn from_physics<N>(r: &Quaternion<N>) -> Quaternion<f32>
    where
        N: PtReal,
    {
        Quaternion::from(Vector4::new(
            N::into(r.i),
            N::into(r.j),
            N::into(r.k),
            N::into(r.w),
        ))
    }
}

pub struct TransfConversor;

impl TransfConversor {
    pub fn to_physics<N>(t: &Isometry3<f32>) -> Isometry3<N>
    where
        N: PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(&t.rotation)),
        )
    }

    pub fn from_physics<N>(t: &Isometry3<N>) -> Isometry3<f32>
    where
        N: PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::from_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::from_physics(&t.rotation)),
        )
    }
}
