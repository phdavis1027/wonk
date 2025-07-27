use std::fmt::Debug;

use super::xml::xml_deserializable::XMLDeserializable;

pub(crate) trait Deserializable: XMLDeserializable + Debug {}
