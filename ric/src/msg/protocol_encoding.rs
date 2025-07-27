use crate::error::IrodsError;
use crate::msg::{deserializable::Deserializable, serializable::Serializable};

use super::irods_prot::IrodsProt;

pub(crate) trait ProtocolEncodingPrivate {
    fn as_enum() -> IrodsProt;

    fn encode_private<M>(msg: &M, sink: &mut Vec<u8>) -> Result<usize, IrodsError>
    where
        M: Serializable;

    fn decode_private<M>(src: &[u8]) -> Result<M, IrodsError>
    where
        M: Deserializable;
}

pub trait ProtocolEncoding {
    fn as_enum() -> IrodsProt;

    fn encode<M, V>(msg: &M, sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<Vec<u8>>,
        M: Serializable;

    fn decode<M, V>(src: V) -> Result<M, IrodsError>
    where
        V: AsRef<[u8]>,
        M: Deserializable;
}

impl<T> ProtocolEncoding for T
where
    T: ProtocolEncodingPrivate,
{
    fn as_enum() -> IrodsProt {
        Self::as_enum()
    }

    fn encode<M, V>(msg: &M, sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<Vec<u8>>,
        M: Serializable,
    {
        Self::encode_private(msg, sink.into())
    }

    fn decode<M, V>(src: &[u8]) -> Result<M, IrodsError>
    where
        V: AsMut<Vec<u8>>,
        M: Deserializable,
    {
        Self::decode_private(src.into())
    }
}
