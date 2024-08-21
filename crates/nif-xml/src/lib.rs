pub mod xml;
pub use xml::NifToolsXml;

pub fn from_xml_str(xml: &str) -> anyhow::Result<xml::NifToolsXml> {
    Ok(quick_xml::de::from_str(xml)?)
}
