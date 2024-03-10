use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct NifToolsXml {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "token")]
    tokens: Vec<Token>,

    #[serde(rename = "verattr")]
    verattrs: Vec<VerAttr>,

    #[serde(rename = "version")]
    versions: Vec<Version>,

    #[serde(rename = "module")]
    modules: Vec<Module>,

    #[serde(rename = "basic")]
    basic_types: Vec<BasicType>,

    #[serde(rename = "bitflags")]
    bit_flags: Vec<BitFlags>,

    #[serde(rename = "enum")]
    enums: Vec<Enum>,

    #[serde(rename = "struct")]
    structs: Vec<Struct>,

    #[serde(rename = "niobject")]
    ni_objects: Vec<NiObject>,
}

impl NifToolsXml {
    pub(crate) fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.name == name)
    }

    pub(crate) fn get_basic_type(&self, name: &str) -> Option<&BasicType> {
        self.basic_types.iter().find(|o| o.name == name)
    }

    pub(crate) fn get_bit_flags(&self, name: &str) -> Option<&BitFlags> {
        self.bit_flags.iter().find(|o| o.name == name)
    }

    pub(crate) fn get_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.iter().find(|o| o.name == name)
    }

    pub(crate) fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.structs.iter().find(|o| o.name == name)
    }

    pub(crate) fn get_ni_object(&self, name: &str) -> Option<&NiObject> {
        self.ni_objects.iter().find(|o| o.name == name)
    }

    pub(crate) fn get_basic_type_names(&self) -> Vec<String> {
        self.basic_types.iter().map(|o| o.name.clone()).collect()
    }

    pub(crate) fn get_bit_flags_names(&self) -> Vec<String> {
        self.bit_flags.iter().map(|o| o.name.clone()).collect()
    }

    pub(crate) fn get_enum_names(&self) -> Vec<String> {
        self.enums.iter().map(|o| o.name.clone()).collect()
    }

    pub(crate) fn get_module_struct_names(&self, module_name: &str) -> Vec<String> {
        self.structs
            .iter()
            .filter(|o| o.module.as_ref().map(|m| m == module_name).unwrap_or(false))
            .map(|o| o.name.clone())
            .collect()
    }

    pub(crate) fn get_module_ni_object_names(&self, module_name: &str) -> Vec<String> {
        self.ni_objects
            .iter()
            .filter(|o| o.module.as_ref().map(|m| m == module_name).unwrap_or(false))
            .map(|o| o.name.clone())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Token {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@attrs")]
    attrs: Vec<String>,

    #[serde(default)]
    verexpr: Vec<TokenValue>,
    #[serde(default)]
    condexpr: Vec<TokenValue>,
    #[serde(default)]
    verset: Vec<TokenValue>,
    #[serde(default)]
    default: Vec<TokenValue>,
    #[serde(default)]
    range: Vec<TokenValue>,
    #[serde(default)]
    global: Vec<TokenValue>,
    #[serde(default)]
    operator: Vec<TokenValue>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TokenValue {
    #[serde(rename = "@token")]
    token: String,
    #[serde(rename = "@string")]
    string: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VerAttr {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@access")]
    access: String,
    #[serde(rename = "@index")]
    index: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Version {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@num")]
    num: String,
    #[serde(rename = "@user")]
    user: Option<String>,
    #[serde(rename = "@bsver")]
    bsver: Option<String>,
    #[serde(rename = "@ext")]
    ext: Option<String>,
    #[serde(rename = "@supported")]
    supported: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Module {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@priority")]
    priority: u32,
    #[serde(rename = "@depends", default)]
    depends: Vec<String>,
    #[serde(rename = "@custom")]
    custom: Option<bool>,
}

impl Module {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_dependencies(&self) -> &Vec<String> {
        self.depends.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct BasicType {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@boolean")]
    boolean: Option<bool>,
    #[serde(rename = "@integral")]
    integral: Option<bool>,
    #[serde(rename = "@countable")]
    countable: Option<bool>,
    #[serde(rename = "@size")]
    size: Option<u32>,
    #[serde(rename = "@convertible", default)]
    convertible: Vec<String>,
    #[serde(rename = "$text")]
    description: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BitFlags {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@storage")]
    storage: String,
    #[serde(rename = "option")]
    options: Vec<BitFlagOption>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BitFlagOption {
    #[serde(rename = "@bit")]
    bit: u32,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Enum {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@storage")]
    storage: String,
    #[serde(rename = "option")]
    options: Vec<EnumOption>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct EnumOption {
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Struct {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@size")]
    size: Option<u32>,
    #[serde(rename = "@module")]
    module: Option<String>,
    #[serde(rename = "@versions")]
    versions: Option<String>,
    #[serde(rename = "field")]
    fields: Vec<StructField>,
    #[serde(rename = "$text")]
    description: Option<String>,
}

impl Struct {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_fields(&self) -> &Vec<StructField> {
        &self.fields
    }

    pub(crate) fn get_description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct StructField {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@length")]
    length: Option<String>,
    #[serde(rename = "@default")]
    default: Option<String>,
    #[serde(rename = "@cond")]
    cond: Option<String>,
    #[serde(rename = "@until")]
    until: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "$text")]
    description: Option<String>,
}

impl StructField {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_type(&self) -> &str {
        &self.r#type
    }

    pub(crate) fn get_version_range(&self) -> (&Option<String>, &Option<String>) {
        (&self.since, &self.until)
    }

    pub(crate) fn get_description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub(crate) fn get_length_field(&self) -> Option<&str> {
        self.length.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct NiObject {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@abstract")]
    r#abstract: Option<bool>,
    #[serde(rename = "@inherit")]
    inherit: Option<String>,
    #[serde(rename = "@module")]
    module: Option<String>,
    #[serde(rename = "@versions")]
    versions: Option<String>,
    #[serde(rename = "@until")]
    until: Option<String>,
    #[serde(rename = "@storage")]
    storage: Option<String>,
    #[serde(rename = "$text")]
    description: Option<String>,
    #[serde(rename = "field", default)]
    fields: Vec<NiObjectField>,
}

impl NiObject {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_fields(&self) -> &Vec<NiObjectField> {
        &self.fields
    }

    pub(crate) fn get_base(&self) -> Option<&str> {
        self.inherit.as_ref().map(|s| s.as_str())
    }

    pub(crate) fn get_description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct NiObjectField {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@length")]
    length: Option<String>,
    #[serde(rename = "@width")]
    width: Option<String>,
    #[serde(rename = "@cond")]
    cond: Option<String>,
    #[serde(rename = "@since")]
    since: Option<String>,
    #[serde(rename = "@until")]
    until: Option<String>,
    #[serde(rename = "@template")]
    template: Option<String>,
    #[serde(rename = "@default")]
    default: Option<String>,
    #[serde(rename = "$text")]
    description: Option<String>,
}

impl NiObjectField {
    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_type(&self) -> &str {
        &self.r#type
    }

    pub(crate) fn get_version_range(&self) -> (&Option<String>, &Option<String>) {
        (&self.since, &self.until)
    }

    pub(crate) fn get_description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::NifToolsXml;
    use quick_xml::de::from_str;

    #[test]
    fn it_parses() {
        let xml_text = std::fs::read_to_string("tests/nif.xml").unwrap();
        let xml: NifToolsXml = from_str(&xml_text).unwrap();
        // dbg!(&xml);
    }
}
