use crate::geometry::ElevatedPoint;
use crate::util;
use crate::{Error, ParseElement, Result};

use quick_xml::events::*;
use quick_xml::Reader;
use std::io::BufRead;

create_scan_iter!(AirportScan, AirportHeliport);

#[derive(Debug, Clone)]
pub struct AirportHeliport {
    pub id: String,
    pub designator: String,
    pub name: String,
    pub arp: Option<ElevatedPoint>,
}

impl ParseElement for AirportHeliport {
    element_name!(b"aixm:AirportHeliport");

    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self> {
        let mut buf = Vec::new();
        let mut designator = None;
        let mut name = None;
        let mut arp = None;
        let id = util::get_id(xml, tag)?;

        // Depth dependent tag reconization
        let mut depth = 0;
        loop {
            match xml.read_event(&mut buf)? {
                Event::Start(ref event) if event.name() == ElevatedPoint::element_name() => {
                    arp = Some(ElevatedPoint::parse(xml, event)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:designator" && depth == 2 => {
                    designator = Some(xml.read_text(b"aixm:designator", &mut buf)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:name" && depth == 2 => {
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
            id,
            designator,
            name,
            arp,
        })
    }
}

create_scan_iter!(RunwayScan, Runway);

#[derive(Debug, Clone)]
pub struct Runway {
    pub id: String,
    pub designator: String,
    pub assoc_airport: String,
}

impl ParseElement for Runway {
    element_name!(b"aixm:Runway");

    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self> {
        let mut buf = Vec::new();

        let id = util::get_id(xml, tag)?;

        let mut designator = None;
        let mut assoc_airport = None;
        loop {
            match xml.read_event(&mut buf)? {
                Event::Start(ref event) if event.name() == b"aixm:designator" => {
                    designator = Some(xml.read_text(b"aixm:designator", &mut buf)?);
                }
                Event::Empty(ref event) if event.name() == b"aixm:associatedAirportHeliport" => {
                    assoc_airport = Some(util::get_gml_link(xml, event)?);
                }
                Event::End(ref event) if event.name() == element_name => break,
                Event::Eof => return Err(Error::BadElement),
                _ => (),
            }
            buf.clear();
        }

        let (assoc_airport, designator) = extract_options!(assoc_airport, designator);

        Ok(Runway {
            id,
            designator,
            assoc_airport,
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_airport() {}
}
