use crate::error::IrodsError;

pub(crate) trait XMLDeserializable {
    fn from_xml(sink: &[u8]) -> Result<usize, IrodsError>
    where
        Self: Sized;
}
