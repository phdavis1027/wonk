use std::fmt::Debug;

use super::xml::xml_serializable::XMLSerializable;

pub(crate) trait Serializable: XMLSerializable + Debug {}
