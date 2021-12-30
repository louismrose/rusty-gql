mod enum_file;
mod input_file;
mod interface_file;
mod object_file;
mod scalar_file;
mod union_file;

use futures_util::future::try_join_all;
use rusty_gql::GqlTypeDefinition;
use std::{collections::BTreeMap, io::Error};

use self::{
    enum_file::EnumFile, input_file::InputObjectFile, interface_file::InterfaceFile,
    object_file::ObjectFile, scalar_file::ScalarFile, union_file::UnionFile,
};

use super::{build_file, mod_file::ModFile};

pub async fn create_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut model_file_names = Vec::new();
    let mut interface_file_names = Vec::new();
    let mut input_file_names = Vec::new();
    let mut scalar_file_names = Vec::new();

    for (_, type_def) in type_definitions.iter() {
        if reserved_scalar_names().contains(&type_def.name()) {
            continue;
        }
        let task = create_type_definition_file(type_def);
        futures.push(task);

        match type_def {
            GqlTypeDefinition::Union(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Enum(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Object(v) => model_file_names.push(v.name.clone()),
            GqlTypeDefinition::Interface(v) => interface_file_names.push(v.name.clone()),
            GqlTypeDefinition::InputObject(v) => input_file_names.push(v.name.clone()),
            GqlTypeDefinition::Scalar(v) => scalar_file_names.push(v.name.clone()),
        }
    }

    build_file(ModFile {
        base_path: "model".to_string(),
        file_names: model_file_names,
    })
    .await?;

    build_file(ModFile {
        base_path: "interface".to_string(),
        file_names: interface_file_names,
    })
    .await?;

    build_file(ModFile {
        base_path: "input".to_string(),
        file_names: input_file_names,
    })
    .await?;

    build_file(ModFile {
        base_path: "scalar".to_string(),
        file_names: scalar_file_names,
    })
    .await?;

    try_join_all(futures).await
}

fn reserved_scalar_names() -> Vec<&'static str> {
    vec!["String", "Int", "Float", "Boolean", "ID"]
}

async fn create_type_definition_file(type_def: &GqlTypeDefinition) -> Result<(), Error> {
    match type_def {
        GqlTypeDefinition::Scalar(def) => build_file(ScalarFile { def }).await,
        GqlTypeDefinition::Object(def) => build_file(ObjectFile { def }).await,
        GqlTypeDefinition::Interface(def) => build_file(InterfaceFile { def }).await,
        GqlTypeDefinition::Union(def) => build_file(UnionFile { def }).await,
        GqlTypeDefinition::Enum(def) => build_file(EnumFile { def }).await,
        GqlTypeDefinition::InputObject(def) => build_file(InputObjectFile { def }).await,
    }
}