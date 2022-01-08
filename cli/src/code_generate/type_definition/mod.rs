mod enum_file;
mod input_file;
mod interface_file;
mod object_file;
mod scalar_file;
mod union_file;

use futures_util::future::try_join_all;
use rusty_gql::{GqlInterface, GqlTypeDefinition, Schema};
use std::{collections::BTreeMap, io::Error};

use self::{
    enum_file::EnumFile, input_file::InputObjectFile, interface_file::InterfaceFile,
    object_file::ObjectFile, scalar_file::ScalarFile, union_file::UnionFile,
};

use super::{create_file, mod_file::ModFile, path_str, util::is_gql_primitive_ty};

pub async fn create_type_definition_files(
    schema: &Schema,
    base_path: &str,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    let mut model_names = Vec::new();
    let mut interface_names = Vec::new();
    let mut input_names = Vec::new();
    let mut scalar_names = Vec::new();

    let mut interfaces_map = BTreeMap::new();

    for (_, type_def) in schema.type_definitions.iter() {
        if is_gql_primitive_ty(&type_def.name()) {
            continue;
        }
        let operation_type_names = vec![
            &schema.query_type_name,
            &schema.mutation_type_name,
            &schema.subscription_type_name,
        ];
        let name = type_def.name();
        if operation_type_names.contains(&&name.to_string()) {
            continue;
        }

        match type_def {
            GqlTypeDefinition::Union(v) => model_names.push(v.name.clone()),
            GqlTypeDefinition::Enum(v) => model_names.push(v.name.clone()),
            GqlTypeDefinition::Object(v) => model_names.push(v.name.clone()),
            GqlTypeDefinition::Interface(v) => {
                interfaces_map.insert(v.name.clone(), v.clone());
                interface_names.push(v.name.clone())
            }
            GqlTypeDefinition::InputObject(v) => input_names.push(v.name.clone()),
            GqlTypeDefinition::Scalar(v) => scalar_names.push(v.name.clone()),
        }

        let task = create_type_definition_file(type_def, base_path, interfaces_map.clone());
        futures.push(task);
    }

    create_file(ModFile {
        path: &path_str(vec![base_path, "model"], false),
        struct_names: model_names,
    })
    .await?;

    create_file(ModFile {
        path: &path_str(vec![base_path, "interface"], false),
        struct_names: interface_names,
    })
    .await?;

    create_file(ModFile {
        path: &path_str(vec![base_path, "input"], false),
        struct_names: input_names,
    })
    .await?;

    create_file(ModFile {
        path: &path_str(vec![base_path, "scalar"], false),
        struct_names: scalar_names,
    })
    .await?;

    try_join_all(futures).await
}

async fn create_type_definition_file(
    type_def: &GqlTypeDefinition,
    base_path: &str,
    interfaces_map: BTreeMap<String, GqlInterface>,
) -> Result<(), Error> {
    let file_name = type_def.name();
    match type_def {
        GqlTypeDefinition::Scalar(def) => {
            let path = path_str(vec![base_path, "scalar", &file_name], true);
            create_file(ScalarFile {
                def,
                path: &path,
                file_name: &file_name,
            })
            .await
        }
        GqlTypeDefinition::Object(def) => {
            let path = path_str(vec![base_path, "model", &file_name], true);
            create_file(ObjectFile {
                def,
                path: &path,
                interfaces_map: &interfaces_map,
                file_name: &file_name,
            })
            .await
        }
        GqlTypeDefinition::Interface(def) => {
            let path = path_str(vec![base_path, "interface", &file_name], true);
            create_file(InterfaceFile {
                def,
                path: &path,
                interface_names: &interfaces_map.keys().cloned().collect::<Vec<_>>(),
                file_name: &file_name,
            })
            .await
        }
        GqlTypeDefinition::Union(def) => {
            let path = path_str(vec![base_path, "model", &file_name], true);
            create_file(UnionFile {
                def,
                path: &path,
                file_name: &file_name,
            })
            .await
        }
        GqlTypeDefinition::Enum(def) => {
            let path = path_str(vec![base_path, "model", &file_name], true);
            create_file(EnumFile {
                def,
                path: &path,
                file_name: &file_name,
            })
            .await
        }
        GqlTypeDefinition::InputObject(def) => {
            let path = path_str(vec![base_path, "input", &file_name], true);
            create_file(InputObjectFile {
                def,
                path: &path,
                file_name: &file_name,
            })
            .await
        }
    }
}
