//! This module contains the necessary functions to convert an Amethyst `Transform` `Isometry`, to a
//! physics `Isometry`.
use amethyst_core::math::{Isometry3, Quaternion, Translation3, UnitQuaternion, Vector3, Vector4};

use crate::PtReal;

pub struct VecConversor;

impl VecConversor {
    /// Used to convert an amethyst `Vector3` `Quaternion` to the physics `Vector3`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(v: &Vector3<f32>) -> Vector3<N>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }

    /// Used to convert a physics `Vector3` to the amethyst `Transform` `Vector3`.
    pub fn from_physics<N>(v: &Vector3<N>) -> Vector3<f32>
    where
        N: PtReal,
    {
        Vector3::new(v.x.into(), v.y.into(), v.z.into())
    }
}

pub struct QuatConversor;

impl QuatConversor {
    /// Used to convert an amethyst `Transform` `Quaternion` to the physics `Quaternion`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(r: &Quaternion<f32>) -> Quaternion<N>
    where
        N: PtReal,
    {
        Quaternion::from(Vector4::new(r.i.into(), r.j.into(), r.k.into(), r.w.into()))
    }

    /// Used to convert a physics `Quaternion` to the amethyst `Transform` `Quaternion`.
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
    /// Used to convert an amethyst `Transform` `Isometry` to the physics `Isometry`.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_physics<N>(t: &Isometry3<f32>) -> Isometry3<N>
    where
        N: PtReal,
    {
        Isometry3::from_parts(
            Translation3::from(VecConversor::to_physics(&t.translation.vector)),
            UnitQuaternion::new_normalize(QuatConversor::to_physics(&t.rotation)),
        )
    }

    /// Used to convert a physics `Isometry` to the amethyst `Transform` `Isometry`.
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
