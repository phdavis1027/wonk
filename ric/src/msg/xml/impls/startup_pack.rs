use crate::{
    error::IrodsError, msg::{
        serializable::Serializable,
        types::startup_pack::StartupPack,
        xml::{macro_rules::macro_imports::*, xml_serializable::XMLSerializable},
    }, tag_end, tag_fmt, tag_start, tag
};

impl<'de_buf> XMLSerializable for StartupPack<'de_buf> {
    fn to_xml<M>(&self, sink: &mut Vec<u8>) -> Result<usize, IrodsError>
    where
        M: Serializable,
    {
        let mut cursor = Cursor::new(sink);
        let mut writer = Writer::new(cursor);

        tag_start!(writer, "StartupPack_PI");

        let irods_prot: &str = (&self.irods_prot).into();

        tag!(writer, "irodsProt", irods_prot);
        tag_fmt!(writer, "reconnFlag", "{}", self.reconn_flag);
        tag_fmt!(writer, "connectCnt", "{}", self.connect_cnt);

        tag!(
            writer, 
            "proxyUser", 
            self.proxy_user.as_ref()
        );

        tag_end!(writer, "StartupPack_PI");

        Ok(cursor.position() as usize)
    }
}
