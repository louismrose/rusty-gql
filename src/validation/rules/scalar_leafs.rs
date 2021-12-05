use graphql_parser::query::Field;

use crate::{
    validation::visitor::{ValidationContext, Visitor},
    GqlTypeDefinition,
};

pub struct ScalarLeafs;

impl<'a> Visitor<'a> for ScalarLeafs {
    fn enter_field(&mut self, ctx: &mut ValidationContext, field: &'a Field<'a, String>) {
        if let Some(parent_type) = ctx.parent_type() {
            let parent = ctx
                .schema
                .type_definitions
                .get(&GqlTypeDefinition::type_name_from_def(parent_type));

            if let Some(parent_gql_type) = parent {
                if let Some(target_field) = parent_gql_type.get_field_by_name(&field.name) {
                    let target = ctx.schema.type_definitions.get(&target_field.name);

                    if let Some(f) = target {
                        if f.is_leaf_type() && !field.selection_set.items.is_empty() {
                            ctx.add_error(
                        format!("Field {} must not have a selection items because type {} has no subfields", &field.name, f.to_string()),
                        vec![field.position])
                        } else if !f.is_leaf_type() && field.selection_set.items.is_empty() {
                            ctx.add_error(
                                format!("Field {} must have selection items", &field.name),
                                vec![field.position],
                            )
                        }
                    }
                }
            }
        }
    }
}
