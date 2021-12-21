use std::collections::HashMap;

use async_trait::async_trait;
use futures_util::future::BoxFuture;
use graphql_parser::{query::Field, schema::Type};
use serde_json::Number;

use crate::{
    context::{FieldContext, SelectionSetContext},
    GqlTypeDefinition, GqlValue, ResolverResult, Schema,
};

pub type ResolverFuture<'a> = BoxFuture<'a, ResolverResult<(String, GqlValue)>>;

#[async_trait]
pub trait SelectionSetResolver: Resolver {
    async fn resolve_selection_set(
        &self,
        ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue>;
}

#[async_trait]
pub trait Resolver: Send + Sync {
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>>;
}

#[async_trait::async_trait]
impl<T: Resolver> Resolver for &T {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    async fn resolve_field(&self, ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        T::resolve_field(*self, ctx).await
    }
}

#[async_trait::async_trait]
impl Resolver for str {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.to_string())))
    }
}

#[async_trait::async_trait]
impl Resolver for String {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::String(self.clone())))
    }
}

#[async_trait::async_trait]
impl Resolver for i8 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for i16 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for i32 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for i64 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for u8 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for u16 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for u32 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for u64 {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for usize {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}

#[async_trait::async_trait]
impl Resolver for isize {
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        Ok(Some(GqlValue::Number(Number::from(*self))))
    }
}
