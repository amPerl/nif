use std::collections::HashSet;

use crate::version::is_in_version_range;

mod version;
mod xml;

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

#[derive(Clone, Default)]
struct RustWriter {
    spaces: usize,
    output: String,
}

impl RustWriter {
    pub fn indent(&mut self) {
        self.spaces += 4;
    }

    pub fn dedent(&mut self) {
        self.spaces -= 4;
    }

    pub fn write_indent(&mut self) {
        self.output.push_str(&" ".repeat(self.spaces));
    }

    pub fn writeln(&mut self, line: &str) {
        self.write_indent();
        self.output.push_str(line);
        self.output.push_str("\n");
    }

    pub fn write_comment(&mut self, comment: &str) {
        for line in comment.lines() {
            self.writeln(&format!("// {}", line.trim_start()));
        }
    }

    pub fn write_doc_comment(&mut self, comment: &str) {
        for line in comment.lines() {
            self.writeln(&format!("/// {}", line.trim_start()));
        }
    }

    pub fn write_block_comment(&mut self, comment: &str) {
        self.writeln("/*");
        for line in comment.lines() {
            self.writeln(&format!(" * {}", line));
        }
        self.writeln(" */");
    }

    pub fn finish(self) -> String {
        self.output
    }
}

pub struct NifCodegen {
    nifxml: xml::NifToolsXml,
    options: NifCodegenOptions,
    writer: RustWriter,
    generated_types: HashSet<String>,
}

impl NifCodegen {
    pub fn from_xml_str(xml: &str, options: NifCodegenOptions) -> anyhow::Result<Self> {
        let xml: xml::NifToolsXml = quick_xml::de::from_str(xml)?;
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
        self.writer
            .writeln(&format!("pub struct {} {{", struct_name));
        self.writer.indent();

        for field in nif_struct.get_fields() {
            let ver_range = field.get_version_range();
            let is_in_range = is_in_version_range(&self.options.desired_version, ver_range);

            let field_name = rustify_name(field.get_name());

            let mut field_type = field.get_type().to_string();
            if let Some(rust_type) = nif_to_rust_type(&field_type) {
                field_type = rust_type;
            } else {
                if !self.generated_types.contains(&field_type) {
                    dbg!(&field_type);
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
                    self.writer
                        .writeln(&format!("#[br(count = {})]", rustify_name(length_field)));
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

            let field_name = rustify_name(field.get_name());
            let mut field_type = field.get_type().to_string();
            if let Some(rust_type) = nif_to_rust_type(&field_type) {
                field_type = rust_type;
            } else {
                if !self.generated_types.contains(&field_type) {
                    dbg!(&field_type);
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

    fn write_module(&mut self, module_name: &str) {
        eprintln!("write_module: {:?}", module_name);
        self.writer.writeln(&format!("mod {} {{", module_name));

        for struct_name in self.nifxml.get_module_struct_names(module_name) {
            self.write_struct(struct_name);
        }

        for ni_object_name in self.nifxml.get_module_ni_object_names(module_name) {
            self.write_ni_object(ni_object_name);
        }

        self.writer.writeln("}");
    }

    pub fn generate(mut self) -> String {
        let mut generated_modules: HashSet<String> = HashSet::new();

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

fn rustify_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

fn nif_to_rust_type(nif_type: &str) -> Option<String> {
    Some(match nif_type {
        "int" => "i32".into(),
        "uint" => "u32".into(),
        "ushort" => "u16".into(),
        "byte" => "u8".into(),
        "ubyte" => "u8".into(),
        "float" => "f32".into(),
        "bool" => "bool".into(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates() {
        let xml_text = std::fs::read_to_string("tests/nif.xml").unwrap();
        let options = NifCodegenOptions::new("20.0.0.4", &["NiMain"]);
        let codegen = NifCodegen::from_xml_str(&xml_text, options).unwrap();
        let result = codegen.generate();
        std::fs::write("tmp/nif.rs", &result).unwrap();
    }
}
