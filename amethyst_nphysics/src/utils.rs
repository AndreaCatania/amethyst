
use crate::storage::StoreTag;

/// Returns the valid reference or fail
#[macro_export]
macro_rules! storage_safe_get {
    ($storage:ident, $tag:expr) => {
        {
            let option = $storage.get(*$tag);
            fail_cond!(option.is_none());
            option.unwrap()
        }
    };
    ($storage:ident, $tag:expr, $fail_ret:expr) => {
        {
            let option = $storage.get(*$tag);
            fail_cond!(option.is_none(), $fail_ret);
            option.unwrap()
        }
    }
}

/// Returns the valid mutable reference or fail
#[macro_export]
macro_rules! storage_safe_get_mut {
    ($storage:ident, $tag:expr) => {
        {
            let option = $storage.get_mut(*$tag);
            fail_cond!(option.is_none());
            option.unwrap()
        }
    };
    ($storage:ident, $tag:expr, $fail_ret:expr) => {
        {
            let option = $storage.get_mut(*$tag);
            fail_cond!(option.is_none(), $fail_ret);
            option.unwrap()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ObjectType {
    RigidBody,
    Area,
}

#[derive(Clone, Debug)]
pub(crate) struct UserData{
    object_type: ObjectType,
    store_tag: StoreTag,
}

impl UserData {
    pub(crate) fn new(object_type: ObjectType, store_tag: StoreTag) -> Self {
        UserData {
            object_type,
            store_tag,
        }
    }
}

impl UserData{
    pub fn object_type(&self) -> ObjectType {self.object_type}
    pub fn store_tag(&self) -> StoreTag {self.store_tag}
}