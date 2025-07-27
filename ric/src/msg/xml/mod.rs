use xml_deserializable::XMLDeserializable;
use xml_serializable::XMLSerializable;

use super::deserializable::Deserializable;
use super::protocol_encoding::ProtocolEncodingPrivate;
use super::serializable::Serializable;

use crate::msg::irods_prot::IrodsProt;

use crate::error::IrodsError;

pub(crate) mod impls;
pub(crate) mod xml_deserializable;
pub(crate) mod xml_serializable;
pub(crate) mod macro_rules;

pub struct XML;

impl ProtocolEncodingPrivate for XML {
    fn as_enum() -> IrodsProt {
        IrodsProt::XML
    }

    fn encode_private<M>(msg: &M, sink: &mut Vec<u8>) -> Result<usize, IrodsError>
    where
        M: Serializable,
    {
        XMLSerializable::to_xml(msg, sink)
    }

    fn decode_private<M>(sink: &[u8]) -> Result<M, IrodsError>
    where
        M: Deserializable,
    {
        XMLDeserializable::from_xml(sink)
    }
}
