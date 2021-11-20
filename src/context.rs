use crate::{
    error::GqlError,
    operation::{ArcOperation, Operation},
    path::GraphQLPath,
    types::{schema::ArcSchema, value::value_from_ast, GqlType},
    GqlValue, Schema,
};
use graphql_parser::{
    query::{Field, Selection, SelectionSet},
    schema::Type,
};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub current_field: Field<'a, String>,
    pub current_path: GraphQLPath,
    pub errors: Vec<GqlError>,
}

pub fn build_context<'a>(
    schema: &'a ArcSchema,
    operation: &'a ArcOperation<'a>,
) -> ExecutionContext<'a> {
    let operation_type = operation.operation_type.to_string();
    let root_fieldname = operation.root_field.name.to_string();
    let current_field = operation.root_field.clone();

    let current_path = GraphQLPath::default()
        .prev(None)
        .current_key(root_fieldname)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        current_field,
        current_path,
        errors: vec![],
    }
}

pub fn get_variables<'a>(
    schema: &'a Schema,
    operation: &'a Operation<'a>,
    input_values: &BTreeMap<String, GqlValue>,
) -> Result<HashMap<String, GqlValue>, String> {
    let variable_definitions = &operation.variable_definitions;
    let mut variables = HashMap::new();
    for var in variable_definitions {
        let var_type = get_type_from_schema(schema, &var.var_type);
        if var_type.is_none() {
            continue;
        }
        let var_type = var_type.unwrap();

        let var_name = &var.name.to_string();
        if !input_values.contains_key(var_name) {
            if let Some(value) = &var.default_value {
                variables.insert(
                    var.name.to_string(),
                    value_from_ast(value, &var_type, &None),
                );
            }
        }

        let value = input_values.get(var_name);

        if let GqlType::NonNull(_) = var_type {
            if value.is_none() {
                return Err(format!("{} must not be null", var_name));
            }
        }

        if let Some(var_value) = value {
            variables.insert(var_name.to_string(), var_value.clone());
        }
    }
    Ok(variables)
}

pub fn get_arguments<'a>(field: Field<'a, String>, variable_values: HashMap<String, GqlValue>) {
    let arguments = field.arguments;
}

pub fn get_type_from_schema<'a>(
    schema: &'a Schema,
    var_type: &'a Type<'a, String>,
) -> Option<GqlType> {
    match var_type {
        graphql_parser::schema::Type::NamedType(named_type) => {
            return schema
                .type_map
                .get(&named_type.to_string())
                .map(|var_ty| var_ty.clone())
        }
        graphql_parser::schema::Type::ListType(list) => {
            let inner_type = get_type_from_schema(schema, &list).unwrap();
            let value = GqlType::List(Box::new(inner_type.clone()));
            return Some(value);
        }
        graphql_parser::schema::Type::NonNullType(non_null) => {
            let inner_type = get_type_from_schema(schema, &non_null).unwrap();
            let value = GqlType::NonNull(Box::new(inner_type.clone()));
            return Some(value);
        }
    }
}

// TODO: schemaはfragmentの条件やskip directiveの処理で使用する
pub fn collect_query_fields<'a>(
    ctx: &'a ExecutionContext<'a>,
    selection_set: &SelectionSet<'a, String>,
) -> HashMap<String, Vec<Field<'a, String>>> {
    let mut fields: HashMap<String, Vec<Field<String>>> = HashMap::new();
    let mut visited_fragments = HashSet::new();

    collect_fields(&ctx, &selection_set, &mut fields, &mut visited_fragments);
    fields
}

fn collect_fields<'a>(
    ctx: &'a ExecutionContext<'a>,
    selection_set: &SelectionSet<'a, String>,
    fields: &mut HashMap<String, Vec<Field<'a, String>>>,
    visited_fragments: &mut HashSet<String>,
) {
    for item in &selection_set.items {
        match item {
            Selection::Field(field) => {
                if fields.contains_key(&field.name.to_string()) {
                    fields
                        .get_mut(&field.name.to_string())
                        .unwrap()
                        .push(field.clone());
                } else {
                    fields.insert(field.name.to_string(), vec![field.clone()]);
                }
            }
            Selection::FragmentSpread(spread_frg) => {
                let fragment_name = &spread_frg.fragment_name;
                if visited_fragments.contains(fragment_name) {
                    continue;
                }
                visited_fragments.insert(fragment_name.to_string());
                let fragment = &ctx.operation.fragments.get(fragment_name);
                match fragment {
                    Some(frg) => {
                        return collect_fields(&ctx, &frg.selection_set, fields, visited_fragments);
                    }
                    None => continue,
                }
            }
            Selection::InlineFragment(inline_frg) => {
                collect_fields(&ctx, &inline_frg.selection_set, fields, visited_fragments);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::collect_query_fields,
        operation::{build_operation, ArcOperation},
        types::schema::{build_schema, ArcSchema},
    };
    use std::fs;

    use super::build_context;

    #[test]
    fn it_works() {
        let schema_doc = fs::read_to_string("src/tests/github.graphql").unwrap();
        let query_doc = fs::read_to_string("src/tests/github_query.graphql").unwrap();

        let schema = ArcSchema::new(build_schema(schema_doc.as_str()).unwrap());
        let query = build_operation(query_doc.as_str(), &schema, None).unwrap();

        let operation = build_operation(&query_doc, &schema, None).unwrap();
        let operation = ArcOperation::new(operation);
        let context = build_context(&schema, &operation);

        let fields = collect_query_fields(&context, &query.selection_set);

        for field in &fields {
            println!("{:?}", field);
        }

        for f in &fields["repository"] {
            for item in &f.selection_set.items {
                println!("{:?}", item);
            }
        }
    }
}
