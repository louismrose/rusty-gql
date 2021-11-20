use std::{ops::Deref, sync::Arc};

use crate::{
    types::schema::{build_schema, ArcSchema},
    Resolver,
};

pub struct Container<Query: Resolver, Mutation: Resolver, Subscription: Resolver> {
    pub query_resolvers: Query,
    pub mutation_resolvers: Mutation,
    pub subscription_resolvers: Subscription,
    pub schema: ArcSchema,
}

pub struct ArcContainer<Query: Resolver, Mutation: Resolver, Subscription: Resolver>(
    Arc<Container<Query, Mutation, Subscription>>,
);

impl<Query: Resolver, Mutation: Resolver, Subscription: Resolver> Deref
    for ArcContainer<Query, Mutation, Subscription>
{
    type Target = Container<Query, Mutation, Subscription>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Query, Mutation, Subscription> ArcContainer<Query, Mutation, Subscription>
where
    Query: Resolver,
    Mutation: Resolver,
    Subscription: Resolver,
{
    pub fn new(
        schema_doc: &str,
        query: Query,
        mutation: Mutation,
        subscription: Subscription,
    ) -> Result<Self, String> {
        let schema = build_schema(schema_doc)?;
        Ok(ArcContainer(Arc::new(Container {
            query_resolvers: query,
            mutation_resolvers: mutation,
            subscription_resolvers: subscription,
            schema: ArcSchema::new(schema),
        })))
    }
}
