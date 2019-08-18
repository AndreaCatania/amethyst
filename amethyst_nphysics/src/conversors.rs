use amethyst_core::{
    components::Transform,
    math::{Quaternion, Translation3, UnitQuaternion, Vector3, Vector4},
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

// TODO think a better way to have an opaque that is the same with all backends
// bug give the possibility to each backend to easily convert itw IDS to the phythyst opaque ID.
/// ```rust
/// use crate::amethyst_nphysics::storage::StoreKey;
/// use crate::amethyst_nphysics::conversors::*;
/// let sk = StoreKey::new(0, 0);
/// assert_eq!(tag_to_store_key(std::num::NonZeroU64::new(1).unwrap()), sk);
/// ```
pub fn tag_to_store_key(tag: std::num::NonZeroU64) -> StoreKey {
    let opaque = tag.get() - 1;
    let index = (opaque & 0xffff_ffff) as usize;
    let generation = opaque >> 32;
    StoreKey::new(index, generation)
}

// TODO think a better way to have an opaque that is the same with all backends
// bug give the possibility to each backend to easily convert itw IDS to the phythyst opaque ID.
/// ```rust
/// use crate::amethyst_nphysics::storage::StoreKey;
/// use crate::amethyst_nphysics::conversors::*;
/// let sk = StoreKey::new(0, 0);
/// assert_eq!(std::num::NonZeroU64::new(1).unwrap(), store_key_to_tag(sk));
/// ```
pub fn store_key_to_tag(key: StoreKey) -> std::num::NonZeroU64 {
    key.map(|index, generation| {
        std::num::NonZeroU64::new((generation << 32 | (index as u64)) + 1).unwrap()
    })
}
