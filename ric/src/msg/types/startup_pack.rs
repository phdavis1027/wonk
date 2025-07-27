use std::borrow::Cow;

use crate::msg::{irods_prot::IrodsProt, serializable::Serializable};

#[derive(Debug)]
pub struct StartupPack<'de_buf> {
    pub irods_prot: IrodsProt,
    pub reconn_flag: u32,
    pub connect_cnt: u32,
    pub proxy_user: Cow<'de_buf, str>,
    pub proxy_zone: Cow<'de_buf, str>,
    pub client_user: Cow<'de_buf, str>,
    pub client_zone: Cow<'de_buf, str>,
    pub rel_version: (u8, u8, u8),
    pub option: Cow<'de_buf, str>,
}

impl<'de_buf> Serializable for StartupPack<'de_buf> {}
