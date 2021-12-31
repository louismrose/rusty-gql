use heck::ToSnakeCase;

use crate::code_generate::FileDefinition;

use super::file_path_str;

pub struct ModFile<'a> {
    pub file_names: Vec<String>,
    pub path: &'a str,
}

impl<'a> FileDefinition for ModFile<'a> {
    fn content(&self) -> String {
        let mut mod_str = String::from("");
        let mut pub_use_str = String::from("");
        for name in &self.file_names {
            mod_str += format!("mod {};\n", &name.to_snake_case()).as_str();
            pub_use_str +=
                format!("pub use {}::{};\n", &name.to_snake_case().as_str(), &name).as_str();
        }

        format!("{}\n{}", mod_str, pub_use_str)
    }

    fn path(&self) -> String {
        file_path_str(vec![self.path, "mod"])
    }
}
