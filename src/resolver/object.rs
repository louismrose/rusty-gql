use std::collections::{BTreeMap, HashMap};

use serde::Serialize;

use crate::{
    types::value::serialize_to_gql_value, FieldContext, GqlValue, Resolver, ResolverResult,
    SelectionSetContext, SelectionSetResolver,
};

#[async_trait::async_trait]
impl<K, V> Resolver for BTreeMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_to_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(Some(GqlValue::Object(map)))
    }
}

#[async_trait::async_trait]
impl<K, V> SelectionSetResolver for BTreeMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_to_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(GqlValue::Object(map))
    }
}

#[async_trait::async_trait]
impl<K, V> Resolver for HashMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_field(&self, _ctx: &FieldContext<'_>) -> ResolverResult<Option<GqlValue>> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_to_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(Some(GqlValue::Object(map)))
    }
}

#[async_trait::async_trait]
impl<K, V> SelectionSetResolver for HashMap<K, V>
where
    K: ToString + Eq + Send + Sync,
    V: Serialize + Send + Sync,
{
    async fn resolve_selection_set(
        &self,
        _ctx: &SelectionSetContext<'_>,
    ) -> ResolverResult<GqlValue> {
        let mut map = BTreeMap::new();
        for (name, v) in self {
            map.insert(
                name.to_string(),
                serialize_to_gql_value(v).unwrap_or_default(),
            );
        }
        Ok(GqlValue::Object(map))
    }
}