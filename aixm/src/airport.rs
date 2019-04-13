use crate::geometry::{AptElevatedPoint, ElevatedPoint};
use crate::{Error, ParseElement, Result};
use std::result::Result as StdResult;

use quick_xml::events::*;
use quick_xml::Reader;
use std::io::BufRead;

pub struct Airports<B: BufRead> {
    xml: Reader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> Airports<B> {
    pub fn from_reader(reader: B) -> Self {
        Airports {
            xml: Reader::from_reader(reader),
            buf: Vec::new(),
        }
    }
}

impl<B: BufRead> Iterator for Airports<B> {
    type Item = Result<AirportHeliport>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buf.clear();
            match self.xml.read_event(&mut self.buf) {
                Ok(Event::Start(ref event)) if event.name() == AirportHeliport::element_name() => {
                    return Some(AirportHeliport::parse(&mut self.xml, event))
                }
                Ok(Event::Eof) => return None,
                Ok(_) => (),
                Err(_) => return None,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AirportHeliport {
    pub id: String,
    pub designator: String,
    pub name: String,
    pub arp: Option<ElevatedPoint>,
}

impl ParseElement for AirportHeliport {
    element_name!(b"aixm:AirportHeliportTimeSlice");

    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self> {
        let mut buf = Vec::new();
        let mut designator = None;
        let mut name = None;
        let mut arp = None;

        // Depth dependent tag reconization
        let mut depth = 0;
        loop {
            match xml.read_event(&mut buf)? {
                Event::Start(ref event) if event.name() == ElevatedPoint::element_name() => {
                    arp = Some(ElevatedPoint::parse(xml, event)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:designator" && depth == 0 => {
                    designator = Some(xml.read_text(b"aixm:designator", &mut buf)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:name" && depth == 0 => {
                    name = Some(xml.read_text(b"aixm:name", &mut buf)?);
                }
                Event::End(ref event) if event.name() == element_name => break,
                Event::Start(_) => {
                    depth += 1;
                }
                Event::End(_) => {
                    depth -= 1;
                }
                Event::Eof => return Err(Error::BadElement),
                _ => (),
            }
            buf.clear();
        }
        let (designator, name) = extract_options!(designator, name);

        Ok(AirportHeliport {
            designator,
            name,
            arp,
        })
    }
}
