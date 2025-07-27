use std::io::Read;

use crate::{error::IrodsError, msg::serializable::Serializable};

pub(crate) trait XMLSerializable {
    fn to_xml<M, R>(&self, sink: R) -> Result<usize, IrodsError>
    where
        R: AsMut<[u8]>,
        M: Serializable;
}
