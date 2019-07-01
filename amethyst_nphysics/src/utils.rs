
use crate::storage::StoreTag;

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