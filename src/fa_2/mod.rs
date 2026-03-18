pub mod builder;
pub mod models;

use std::error::Error;
use xmltree::{Element, EmitterConfig};

pub fn pretty_print_xml(xml: &str) -> Result<String, Box<dyn Error>> {
    let body = if xml.starts_with("<?xml") {
        match xml.find("?>") {
            Some(p) => &xml[p + 2..],
            None => xml,
        }
    } else {
        xml
    };

    let root = Element::parse(body.as_bytes())?;
    let mut buffer = Vec::new();
    let config = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false);
    root.write_with_config(&mut buffer, config)?;
    let pretty_body = String::from_utf8(buffer)?;

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n{}",
        pretty_body
    ))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
