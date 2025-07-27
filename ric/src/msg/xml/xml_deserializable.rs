use crate::error::IrodsError;

pub(crate) trait XMLDeserializable {
    fn from_xml_owning(sink: &[u8]) -> Result<Self, IrodsError>
    where
        Self: 'static + Sized;

    fn from_xml<'de_buf>(sink: &'de_buf [u8]) -> Result<Self, IrodsError>
    where
        Self: 'de_buf + Sized;
}
