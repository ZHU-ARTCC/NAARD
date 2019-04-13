use crate::{Error, ParseElement, Result};
use std::result::Result as StdResult;

use quick_xml::events::*;
use quick_xml::Reader;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl ParseElement for Point {
    element_name!(b"gml:pos");

    fn parse_inner<B: BufRead>(xml: &mut Reader<B>, element_name: &'static [u8]) -> Result<Self> {
        let mut buf = Vec::new();
        let text = xml.read_text(element_name, &mut buf)?;

        let split: StdResult<Vec<_>, _> =
            text.split_whitespace().take(2).map(|s| s.parse()).collect();

        if let Ok(split) = split {
            if split.len() == 2 {
                Ok(Point {
                    x: split[0],
                    y: split[1],
                })
            } else {
                Err(Error::BadElement)
            }
        } else {
            Err(Error::BadElement)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElevatedPoint {
    pub point: Option<Point>,
    pub elevation: Option<f32>,
}

impl ParseElement for ElevatedPoint {
    element_name!(b"aixm:ElevatedPoint");

    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self> {
        let mut point = None;
        let mut elevation = None;
        let mut buf = Vec::new();
        loop {
            match xml.read_event(&mut buf)? {
                Event::Start(ref event) if event.name() == Point::element_name() => {
                    point = Some(Point::parse(xml, event)?);
                }
                Event::Start(ref event) if event.name() == b"aixm:elevation" => {
                    elevation = Some(
                        xml.read_text(b"aixm:elevation", &mut buf)?
                            .parse()
                            .map_err(|_| Error::BadElement)?,
                    );
                }
                Event::End(ref event) if event.name() == element_name => break,
                Event::Eof => return Err(Error::BadElement),
                _ => (),
            }
            buf.clear();
        }

        Ok(ElevatedPoint { point, elevation })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AptElevatedPoint(pub ElevatedPoint);

impl ParseElement for AptElevatedPoint {
    element_name!(b"apt:ElevatedPoint");

    fn parse_inner<B: BufRead>(
        xml: &mut Reader<B>,
        tag: &BytesStart,
        element_name: &'static [u8],
    ) -> Result<Self> {
        Ok(AptElevatedPoint(ElevatedPoint::parse_inner(
            xml,
            tag,
            element_name,
        )?))
    }
}
