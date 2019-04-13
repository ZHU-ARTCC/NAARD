use std::sync::Arc;
use quick_xml::Reader;
use std::io::BufRead;
use std::fmt::Debug;
use custom_error::custom_error;

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

pub mod airport;
pub mod geometry;

type Result<T> = std::result::Result<T, Error>;

custom_error!{Error
    BadElement = "The definition of the element did not match specfication",
    XML {xml: quick_xml::Error} = "XML Reader/Writer Error"
}

impl From<quick_xml::Error> for Error {
    fn from(xml: quick_xml::Error) -> Error {
        Error::XML { xml }
    }
}

trait ParseElement : Debug + Clone 
where Self: Sized {
    fn element_name() -> &'static [u8];
    fn parse_inner<B: BufRead>(xml: &mut Reader<B>, element_name: &'static [u8]) -> Result<Self>;

    fn parse<B: BufRead>(xml: &mut Reader<B>) -> Result<Self> {
        Self::parse_inner(xml, Self::element_name())
    }
}