use graphql_parser::query::{FragmentDefinition, InlineFragment};

use crate::validation::{
    utils::get_type_name,
    visitor::{ValidationContext, Visitor},
};

pub struct FragmentsOnCompositeTypes;

impl<'a> Visitor<'a> for FragmentsOnCompositeTypes {
    fn enter_fragment_definition(
        &mut self,
        ctx: &mut ValidationContext,
        name: &'a str,
        fragment_definition: &'a FragmentDefinition<'a, String>,
    ) {
        if let Some(current_type) = ctx.current_type() {
            let type_name = get_type_name(current_type);
            let target_type = ctx.schema.type_map.get(&type_name);

            if let Some(ty) = target_type {
                if !ty.is_composite_type() {
                    ctx.add_error(
                        format!("Fragment {} cannot condition non composite type", name),
                        vec![fragment_definition.position],
                    )
                }
            }
        }
    }

    fn enter_inline_fragment(
        &mut self,
        ctx: &mut ValidationContext,
        inline_fragment: &'a InlineFragment<'a, String>,
    ) {
        if let Some(current_type) = ctx.current_type() {
            let type_name = get_type_name(current_type);
            let target_type = ctx.schema.type_map.get(&type_name);

            if let Some(ty) = target_type {
                if !ty.is_composite_type() {
                    ctx.add_error(
                        format!("Fragment {} cannot condition non composite type", type_name),
                        vec![inline_fragment.position],
                    )
                }
            }
        }
    }
}