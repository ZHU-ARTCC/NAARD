use crate::{Error, Result};
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use std::io::BufRead;

pub(crate) fn get_attribute<B: BufRead>(
    reader: &Reader<B>,
    tag: &BytesStart,
    attr: &str,
) -> Result<Option<String>> {
    tag.attributes()
        .flat_map(|x| x)
        .map(|x| (x.key, x.unescape_and_decode_value(reader)))
        .find(|a| a.0 == attr.as_bytes())
        .map(|(_, val)| val)
        .transpose()
        .map_err(Error::from)
}

pub(crate) fn get_id<B: BufRead>(reader: &Reader<B>, tag: &BytesStart) -> Result<String> {
    match get_attribute(reader, tag, "gml:id")? {
        Some(id) => Ok(id),
        None => Err(Error::BadElement),
    }
}

pub(crate) fn get_gml_link<B: BufRead>(reader: &Reader<B>, tag: &BytesStart) -> Result<String> {
    let attr = get_attribute(reader, tag, "xlink:href")?;
    let s = attr.and_then(|a| extract_gml_id(&a).map(ToString::to_string));
    match s {
        Some(s) => Ok(s),
        None => Err(Error::BadElement),
    }
}

pub(crate) fn extract_gml_id(href: &str) -> Option<&str> {
    let s = href.split_at(href.find("@gml:id")?).1;
    let mut data = s.split('\'');
    data.next(); // Dispose of the gml:id part
    Some(data.next()?.trim())
}
