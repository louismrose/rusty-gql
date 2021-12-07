use std::{collections::BTreeMap, io::Error};

use codegen::Scope;
use futures::future::try_join_all;
use rusty_gql::{
    GqlEnum, GqlInputObject, GqlInterface, GqlObject, GqlScalar, GqlTypeDefinition, GqlUnion,
};

use super::utils::{create_file, PathStr};

pub async fn gen_type_definition_files(
    type_definitions: &BTreeMap<String, GqlTypeDefinition>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();
    for (_, type_def) in type_definitions.iter() {
        let task = gen_type_definition_file(type_def);
        futures.push(task);
    }
    let res = try_join_all(futures).await;
    res
}

async fn gen_type_definition_file(type_def: &GqlTypeDefinition) -> Result<(), Error> {
    let path =
        PathStr::new(vec![&type_def.to_string().to_lowercase(), type_def.name()]).to_string();
    if tokio::fs::File::open(&path).await.is_err() {
        let content = gen_type_definition_str(&type_def);
        create_file(&path, &content).await?;
        Ok(())
    } else {
        Ok(())
    }
}

fn gen_type_definition_str(type_def: &GqlTypeDefinition) -> String {
    match type_def {
        GqlTypeDefinition::Scalar(scalar) => gen_scalar_str(scalar),
        GqlTypeDefinition::Object(obj) => gen_object_str(obj),
        GqlTypeDefinition::Interface(interface) => gen_interface_str(interface),
        GqlTypeDefinition::Union(uni) => gen_union_str(uni),
        GqlTypeDefinition::Enum(enu) => gen_enum_str(enu),
        GqlTypeDefinition::InputObject(input) => gen_input_object_str(input),
        GqlTypeDefinition::List(list) => gen_list_str(list),
    }
}

fn gen_scalar_str(gql_scalar: &GqlScalar) -> String {
    let mut scope = Scope::new();
    scope.new_struct(gql_scalar.name.as_str()).vis("pub");

    scope.to_string()
}

fn gen_object_str(gql_obj: &GqlObject) -> String {
    let mut scope = Scope::new();
    let struct_scope = scope.new_struct(gql_obj.name.as_str()).vis("pub");

    for field in &gql_obj.fields {
        struct_scope.field(&field.name, field.meta_type.to_rust_type());
    }

    scope.to_string()
}

fn gen_interface_str(gql_interface: &GqlInterface) -> String {
    let mut scope = Scope::new();
    let trait_scope = scope.new_trait(gql_interface.name.as_str()).vis("pub");

    for field in &gql_interface.fields {
        trait_scope
            .new_fn(&field.name)
            .ret(field.meta_type.to_rust_type());
    }
    scope.to_string()
}

fn gen_union_str(gql_union: &GqlUnion) -> String {
    let mut scope = Scope::new();
    let enum_scope = scope.new_enum(gql_union.name.as_str()).vis("pub");

    for value in &gql_union.types {
        enum_scope.new_variant(&value);
    }

    scope.to_string()
}

fn gen_input_object_str(gql_input: &GqlInputObject) -> String {
    let mut scope = Scope::new();
    let struct_scope = scope.new_struct(gql_input.name.as_str()).vis("pub");

    for field in &gql_input.fields {
        struct_scope.field(&field.name, field.meta_type.to_rust_type());
    }

    scope.to_string()
}

fn gen_list_str(gql_list: &GqlTypeDefinition) -> String {
    let mut scope = Scope::new();
    println!("{}", gql_list.name());

    scope.to_string()
}

fn gen_enum_str(gql_enum: &GqlEnum) -> String {
    let mut scope = Scope::new();
    let enum_scope = scope.new_enum(gql_enum.name.as_str()).vis("pub");

    for value in &gql_enum.values {
        enum_scope.new_variant(&value.name);
    }

    scope.to_string()
}