use crate::{Error, ParseElement, Result};
use crate::geometry::{ElevatedPoint, AptElevatedPoint};
use std::result::Result as StdResult;

use quick_xml::events::*;
use quick_xml::Reader;
use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct AirportHeliport {
    designator: String,
    name: String,
    arp: Option<ElevatedPoint>
}

impl ParseElement for AirportHeliport {
    element_name!(b"aixm:AirportHeliport");

    fn parse_inner<B: BufRead>(xml: &mut Reader<B>, element_name: &'static [u8]) -> Result<Self> {
        let mut buf = Vec::new();
        let mut designator = None;
        let mut name = None;
        let mut arp = None;

        loop {
            match xml.read_event(&mut buf)? {
                Event::Start(ref event) if event.name() == AptElevatedPoint::element_name() => {
                    arp = Some(AptElevatedPoint::parse(xml)?.0);
                }
                Event::Start(ref event) if event.name() == ElevatedPoint::element_name() => {
                    arp = Some(ElevatedPoint::parse(xml)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:designator" => {
                    designator = Some(xml.read_text(b"aixm:designator", &mut buf)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:name" => {
                    name = Some(xml.read_text(b"aixm:name", &mut buf)?);
                }
                Event::End(ref event) if event.name() == element_name => break,
                Event::Eof => return Err(Error::BadElement),
                _ => ()
            }
            buf.clear();
        }

        let (designator, name) = extract_options!(designator, name);

        Ok(AirportHeliport {
            designator,
            name,
            arp
        })

    }
}