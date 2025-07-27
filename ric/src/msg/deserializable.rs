use std::fmt::Debug;

pub(crate) trait Deserializable: XMLDeserializable + Debug {}
