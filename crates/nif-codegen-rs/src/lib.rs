use std::collections::HashSet;

use crate::version::is_in_version_range;

mod version;
mod writer;

use writer::RustWriter;

#[derive(Clone)]
pub struct NifCodegenOptions {
    pub desired_version: String,
    pub desired_modules: Vec<String>,
    pub debug: bool,
}

impl NifCodegenOptions {
    pub fn new(desired_version: &str, desired_modules: &[&str]) -> Self {
        Self {
            desired_version: desired_version.into(),
            desired_modules: desired_modules.iter().map(|s| s.to_string()).collect(),
            debug: true,
        }
    }
}

pub struct NifCodegen {
    nifxml: nif_xml::xml::NifToolsXml,
    options: NifCodegenOptions,
    writer: RustWriter,
    generated_types: HashSet<String>,
}

impl NifCodegen {
    pub fn from_xml_str(xml: &str, options: NifCodegenOptions) -> anyhow::Result<Self> {
        let xml = nif_xml::from_xml_str(xml)?;
        Ok(Self {
            nifxml: xml,
            options,
            writer: Default::default(),
            generated_types: Default::default(),
        })
    }

    fn write_struct(&mut self, struct_name: String) {
        eprintln!("write_struct: {:?}", struct_name);
        let nif_struct = self.nifxml.get_struct(&struct_name).unwrap();

        // add debug comment with pretty printed xml entity
        if self.options.debug {
            self.writer
                .write_block_comment(&format!("{:#?}", nif_struct));
        }

        // if a description exists, add it as a doc comment
        if let Some(description) = nif_struct.get_description() {
            self.writer.write_doc_comment(description);
        }

        // struct header
        self.writer.writeln("#[binrw::binrw]");
        if nif_struct.is_generic() {
            self.writer.writeln(&format!(
                "pub struct {}<T: binrw::BinRead + binrw::BinWrite + 'static>",
                struct_name
            ));
            self.writer.writeln("where");
            self.writer.indent();
            self.writer
                .writeln("T: for<'a> binrw::BinRead<Args<'a> = ()>,");
            self.writer
                .writeln("T: for<'a> binrw::BinWrite<Args<'a> = ()>,");
            self.writer.dedent();
            self.writer.writeln("{");
        } else {
            self.writer
                .writeln(&format!("pub struct {} {{", struct_name));
        }
        self.writer.indent();

        for field in nif_struct.get_fields() {
            let ver_range = field.get_version_range();
            let is_in_range = is_in_version_range(&self.options.desired_version, ver_range);

            let field_name = rustify_field_name(field.get_name());

            let mut field_type = field.get_type().to_string();
            if field_type == "#T#" {
                field_type = "T".into();
            } else if let Some(rust_type) = nif_to_rust_type(&field_type) {
                field_type = rust_type;
            } else if !self.generated_types.contains(&field_type) {
                dbg!(&field_type);
            }
            if let Some(tpl) = field.get_template() {
                if let Some(rust_type) = nif_to_rust_type(tpl) {
                    field_type = format!("{}<{}>", field_type, rust_type);
                } else if !self.generated_types.contains(tpl) {
                    field_type = format!("{}<{}>", field_type, tpl);
                    dbg!(&tpl);
                }
            }
            if field.get_length_field().is_some() {
                field_type = format!("Vec<{}>", field_type);
            }

            let field_str = format!("pub {field_name}: {field_type},");

            if is_in_range {
                // if a description exists, add it as a doc comment
                if let Some(description) = field.get_description() {
                    self.writer.write_doc_comment(description);
                }
                if let Some(length_field) = field.get_length_field() {
                    self.writer.writeln(&format!(
                        "#[br(count = {})]",
                        rustify_field_name(length_field)
                    ));
                }
                self.writer.writeln(&field_str);
            } else if self.options.debug {
                self.writer
                    .write_comment(&format!("field is out of version range {:?}\n", ver_range));
                self.writer.write_comment(&field_str);
            }
        }

        self.writer.dedent();
        self.writer.writeln("}");
    }

    fn write_ni_object(&mut self, ni_object_name: String) {
        eprintln!("write_ni_object: {:?}", ni_object_name);
        let ni_object = self.nifxml.get_ni_object(&ni_object_name).unwrap();

        // add debug comment with pretty printed xml entity
        if self.options.debug {
            self.writer
                .write_block_comment(&format!("{:#?}", ni_object));
        }

        // if a description exists, add it as a doc comment
        if let Some(description) = ni_object.get_description() {
            self.writer.write_doc_comment(description);
        }

        // struct header
        self.writer.writeln("#[binrw::binrw]");
        self.writer
            .writeln(&format!("pub struct {} {{", ni_object.get_name()));
        self.writer.indent();

        // if it has a base class, specify it
        if let Some(base) = ni_object.get_base() {
            self.writer.writeln(&format!("pub base: {},", base));
        }

        for field in ni_object.get_fields() {
            let ver_range = field.get_version_range();
            let is_in_range = is_in_version_range(&self.options.desired_version, ver_range);

            let field_name = rustify_field_name(field.get_name());
            let mut field_type = field.get_type().to_string();
            if let Some(rust_type) = nif_to_rust_type(&field_type) {
                field_type = rust_type;
            } else if !self.generated_types.contains(&field_type) {
                dbg!(&field_type);
            }
            if let Some(tpl) = field.get_template() {
                if let Some(rust_type) = nif_to_rust_type(tpl) {
                    field_type = format!("{}<{}>", field_type, rust_type);
                } else if !self.generated_types.contains(tpl) {
                    field_type = format!("{}<{}>", field_type, tpl);
                    dbg!(&tpl);
                }
            }

            let field_str = format!("pub {field_name}: {field_type},");

            if is_in_range {
                // if a description exists, add it as a doc comment
                if let Some(description) = field.get_description() {
                    self.writer.write_doc_comment(description);
                }
                self.writer.writeln(&field_str);
            } else if self.options.debug {
                self.writer
                    .write_comment(&format!("field is out of version range {:?}\n", ver_range));
                self.writer.write_comment(&field_str);
            }
        }

        self.writer.dedent();
        self.writer.writeln("}");
    }

    fn write_enum(&mut self, enum_name: String) {
        eprintln!("write_enum: {:?}", enum_name);
        let nif_enum = self.nifxml.get_enum(&enum_name).unwrap();

        // add debug comment with pretty printed xml entity
        if self.options.debug {
            self.writer.write_block_comment(&format!("{:#?}", nif_enum));
        }

        // if a description exists, add it as a doc comment
        if let Some(description) = nif_enum.get_description() {
            self.writer.write_doc_comment(description);
        }

        // enum header
        self.writer.writeln("#[binrw::binrw]");
        self.writer
            .writeln(&format!("pub enum {} {{", nif_enum.get_name()));
        self.writer.indent();

        let enum_type = if let Some(rust_type) = nif_to_rust_type(nif_enum.get_storage()) {
            rust_type
        } else {
            dbg!(nif_enum.get_storage());
            nif_enum.get_storage().to_string()
        };

        for option in nif_enum.get_options() {
            let option_name = rustify_enum_option(option.get_name());
            self.writer.writeln(&format!(
                "#[brw(magic = {}_{})]",
                option.get_value(),
                enum_type
            ));
            self.writer.writeln(&format!("{},", option_name,));
        }

        self.writer.dedent();
        self.writer.writeln("}");
    }

    fn write_module(&mut self, module_name: &str) {
        eprintln!("write_module: {:?}", module_name);
        self.writer.writeln(&format!("mod {} {{", module_name));
        self.writer.indent();
        self.writer.writeln("use super::*;");
        self.writer.writeln("use crate::basic_types::*;");

        for struct_name in self.nifxml.get_module_struct_names(module_name) {
            self.write_struct(struct_name);
        }

        for ni_object_name in self.nifxml.get_module_ni_object_names(module_name) {
            self.write_ni_object(ni_object_name);
        }

        self.writer.dedent();
        self.writer.writeln("}");
    }

    pub fn generate(mut self) -> String {
        let mut generated_modules: HashSet<String> = HashSet::new();

        for enum_name in self.nifxml.get_enum_names() {
            self.write_enum(enum_name);
        }

        for module in self.options.desired_modules.clone() {
            eprintln!("desired module name: {:?}", module);
            let xml_module = self.nifxml.get_module(&module).unwrap();
            let xml_module_deps = xml_module.get_dependencies().clone();

            if !generated_modules.contains(&module) {
                self.write_module(&module);
                generated_modules.insert(module);
            }

            for dep_module_name in xml_module_deps {
                if !generated_modules.contains(&dep_module_name) {
                    self.write_module(&dep_module_name);
                    generated_modules.insert(dep_module_name);
                }
            }
        }

        self.writer.finish()
    }
}

fn escape_name(name: String) -> String {
    match name.as_str() {
        "type" => "r#type".into(),
        "box" => "r#box".into(),
        _ => name,
    }
}

fn rustify_field_name(name: &str) -> String {
    escape_name(name.to_lowercase().replace(' ', "_"))
}

fn rustify_enum_option(name: &str) -> String {
    let mut new_name = "".to_string();
    for word in name.split(|c| c == ' ' || c == '_') {
        let first_letter = word.chars().next().unwrap().to_ascii_uppercase();
        let rest = word.chars().skip(1).collect::<String>().to_lowercase();
        new_name.push(first_letter);
        new_name.push_str(&rest);
    }
    new_name
}

fn nif_to_rust_type(nif_type: &str) -> Option<String> {
    Some(match nif_type {
        "byte" | "char" | "ubyte" => "u8".into(),
        "bool" => "u8".into(), // 32 bit in old versions pre 4.0.0.0 or something
        "ushort" => "u16".into(),
        "hfloat" => "Float16".into(),
        "normbyte" => "NormByte".into(),
        "int" => "i32".into(),
        "uint" | "ulittle32" => "u32".into(),
        "float" => "f32".into(),
        "#T#" => "T".into(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let xml_text = std::fs::read_to_string("../nif-xml/tests/nif.xml").unwrap();
        let options = NifCodegenOptions::new("20.0.0.4", &["NiMain"]);
        let codegen = NifCodegen::from_xml_str(&xml_text, options).unwrap();
        let result = codegen.generate();
        std::fs::write("../nif/src/nif.rs", result).unwrap();
    }
}
