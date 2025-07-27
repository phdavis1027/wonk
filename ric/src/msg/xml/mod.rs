use xml_deserializable::XMLDeserializable;
use xml_serializable::XMLSerializable;

use super::deserializable::Deserializable;
use super::protocol_encoding::ProtocolEncodingPrivate;
use super::serializable::Serializable;

use crate::msg::irods_prot::IrodsProt;

use crate::error::IrodsError;

pub(crate) mod impls;
pub(crate) mod macro_rules;
pub(crate) mod xml_deserializable;
pub(crate) mod xml_serializable;

pub struct XML;

impl ProtocolEncodingPrivate for XML {
    fn as_enum() -> IrodsProt {
        IrodsProt::XML
    }

    fn encode_private<M>(msg: &M, sink: &mut [u8]) -> Result<usize, IrodsError>
    where
        M: Serializable,
    {
        XMLSerializable::to_xml::<M>(msg, sink)
    }

    fn decode_private<'de_buf, M>(src: &'de_buf [u8]) -> Result<M, IrodsError>
    where
        M: 'de_buf + Deserializable,
    {
        XMLDeserializable::from_xml(src)
    }

    fn decode_owning_private<M>(src: &[u8]) -> Result<M, IrodsError>
    where
        M: 'static + Deserializable,
    {
        XMLDeserializable::from_xml_owning(src)
    }
}
