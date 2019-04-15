#![deny(clippy::all)]
#![forbid(unsafe_code)]

use custom_error::custom_error;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::fmt::Debug;
use std::io::BufRead;

macro_rules! element_name {
    ($str:expr) => {
        fn element_name() -> &'static [u8] {
            $str
        }
    };
}

macro_rules! extract_options {
    ($($it:ident),+) => {
        match ($($it),+) {
            ($(Some($it)),+) => ($($it),+),
            _ => return Err(Error::BadElement)
        }
    };
}

macro_rules! create_scan_iter {
    ($itername:ident, $sname:ident) => {

        pub struct $itername <B: BufRead> {
            xml: Reader<B>,
            buf: Vec<u8>,
        }

        impl<B: BufRead> $itername<B> {
            pub fn from_reader(reader: B) -> Self {
                $itername {
                    xml: Reader::from_reader(reader),
                    buf: Vec::new(),
                }
            }
        }

        impl<B: BufRead> Iterator for $itername<B> {
            type Item = Result<$sname>;
            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    self.buf.clear();
                    match self.xml.read_event(&mut self.buf) {
                        Ok(Event::Start(ref event)) if event.name() == $sname::element_name() => {
                            return Some($sname::parse(&mut self.xml, event))
                        }
                        Ok(Event::Eof) => return None,
                        Ok(_) => (),
                        Err(_) => return None,
                    }
                }
            }
        }
    }
}

pub mod airport;
pub mod geometry;
mod util;

type Result<T> = std::result::Result<T, Error>;

custom_error! {pub Error
    BadElement = "The definition of the element did not match specfication",
    XML {xml: quick_xml::Error} = "XML Reader/Writer Error"
}

impl From<quick_xml::Error> for Error {
    fn from(xml: quick_xml::Error) -> Error {
        Error::XML { xml }
    }
}

trait ParseElement: Debug + Clone
where
    Self: Sized,
{
    fn element_name() -> &'static [u8];
    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self>;

    fn parse<B: BufRead>(xml: &mut Reader<B>, tag: &BytesStart) -> Result<Self> {
        Self::parse_inner(xml, tag, Self::element_name())
    }
}