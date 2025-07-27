use crate::{error::IrodsError, msg::serializable::Serializable};

pub(crate) trait XMLSerializable {
    fn to_xml<M>(&self, sink: &mut Vec<u8>) -> Result<usize, IrodsError>
    where
        M: Serializable;
}
