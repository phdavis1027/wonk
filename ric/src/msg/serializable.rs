use std::fmt::Debug;

pub(crate) trait Serializable: XMLDeserializable + Debug {}
