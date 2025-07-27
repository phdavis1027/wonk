pub(crate) fn irods_escapes<'entity>(c: u8) -> Option<&'entity [u8]> {
    match c {
        b'<' => Some(b"&lt;"),
        b'>' => Some(b"&gt;"),
        b'\'' => Some(b"&apos;"),
        b'"' => Some(b"&quot;"),
        b'`' => Some(b"&apos;"),
        b'&' => Some(b"&amp;"),
        _ => None,
    }
}
