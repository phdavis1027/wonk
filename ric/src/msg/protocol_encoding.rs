use crate::error::IrodsError;
use crate::msg::{deserializable::Deserializable, serializable::Serializable};

use super::irods_prot::IrodsProt;

pub(crate) trait ProtocolEncodingPrivate {
    fn as_enum() -> IrodsProt;

    fn encode_unbounded_private<M>(msg: &M, sink: &mut Vec<u8>) -> Result<usize, IrodsError>
    where
        M: Serializable;

    fn encode_bounded_private<M, const N: usize>(
        msg: &M,
        sink: &mut [u8; N],
    ) -> Result<usize, IrodsError>
    where
        M: Serializable;

    fn decode_owning_private<M>(src: &[u8]) -> Result<M, IrodsError>
    where
        M: 'static + Deserializable;

    fn decode_private<'de_buf, M>(src: &'de_buf [u8]) -> Result<M, IrodsError>
    where
        M: 'de_buf + Deserializable;
}

pub trait ProtocolEncoding {
    fn as_enum() -> IrodsProt;

    fn encode_unbounded<M, V>(msg: &M, sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<Vec<u8>>,
        M: Serializable;

    fn encode_bounded<M, V, const N: usize>(msg: &M, sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<[u8; N]>,
        M: Serializable;

    fn decode_owning<M, V>(src: V) -> Result<M, IrodsError>
    where
        V: AsRef<[u8]>,
        M: 'static + Deserializable;

    fn decode<'de_buf, M, V>(src: V) -> Result<M, IrodsError>
    where
        V: 'de_buf + AsRef<[u8]>,
        M: 'de_buf + Deserializable;
}

impl<T> ProtocolEncoding for T
where
    T: ProtocolEncodingPrivate,
{
    fn as_enum() -> IrodsProt {
        Self::as_enum()
    }

    fn encode_unbounded<M, V>(msg: &M, mut sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<Vec<u8>>,
        M: Serializable,
    {
        Self::encode_unbounded_private(msg, sink.as_mut())
    }

    fn encode_bounded<M, V, const N: usize>(msg: &M, mut sink: V) -> Result<usize, IrodsError>
    where
        V: AsMut<[u8; N]>,
        M: Serializable,
    {
        Self::encode_bounded_private(msg, sink.as_mut())
    }

    fn decode_owning<M, V>(src: V) -> Result<M, IrodsError>
    where
        V: AsRef<[u8]>,
        M: 'static + Deserializable,
    {
        Self::decode_owning_private(src.as_ref())
    }

    fn decode<'de_buf, M, V>(src: V) -> Result<M, IrodsError>
    where
        V: 'de_buf + AsRef<[u8]>,
        M: 'de_buf + Deserializable,
    {
        Self::decode_private(src.as_ref())
    }
}
