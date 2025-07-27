pub(crate) mod macro_imports {
    pub(crate) use std::io::{Write, Cursor};
    pub(crate) use quick_xml::{events::{
        Event,
        BytesStart,
        BytesEnd,
        BytesText,
    }, writer::Writer};
}

#[macro_export]
macro_rules! tag {
    ($writer:ident, $name:expr, $value:expr) => {
        $writer.write_event(Event::Start(BytesStart::new($name)))?; 
        $writer.write_event(Event::Text(BytesText::new($value)))?;
        $writer.write_event(Event::End(BytesEnd::new($name)))?;
    };
}

#[macro_export]
macro_rules! tag_fmt {
    ($writer:ident, $name:expr, $fmt_str:expr, $($value:expr),*) => {
        $writer.write_event(Event::Start(BytesStart::new($name)))?;
        write!($writer.get_mut(), $fmt_str, $($value),*)?;
        $writer.write_event(Event::End(BytesEnd::new($name)))?;
    };
}

#[macro_export]
macro_rules! tag_start {
    ($writer:ident, $name:expr) => {
        $writer.write_event(Event::Start(BytesStart::new($name)))?;
    };
}

#[macro_export]
macro_rules! tag_end {
    ($writer:ident, $name:expr) => {
        $writer.write_event(Event::End(BytesEnd::new($name)))?;
    };
}

#[macro_export]
macro_rules! escape {
    ($value:expr, $escape:ident) => {
        
    }
}
