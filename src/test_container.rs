use std::{collections::HashMap, pin::Pin};

use futures::Future;

use crate::{graphql_value::GraphQLValue, GraphQLResponse};

// Container holds the information of the resolver
// なぜBoxFuture型にするかというとtraitで非同期関数を定義していて、そこの戻り値はBox dyn Futureを返すため、BoxFutureを使用している
type GraphQLFuture<'a> = Pin<Box<dyn Future<Output = GraphQLResponse<GraphQLValue>> + 'a + Send>>;

pub struct Container<'a> {
    // key is parent_type and target field
    // value is resolve fn
    resolvers: HashMap<(&'a str, &'a str), GraphQLFuture<'a>>,
}

#[async_trait::async_trait]
trait Resolver: Send + Sync {
    async fn resolve(&self) -> GraphQLResponse<GraphQLValue>;
}

struct Query;

#[async_trait::async_trait]
impl Resolver for Query {
    async fn resolve(&self) -> GraphQLResponse<GraphQLValue> {
        Ok(GraphQLValue::String(String::from("resolve")))
    }
}

impl Query {
    pub async fn test_async(&self) -> GraphQLResponse<GraphQLValue> {
        Ok(GraphQLValue::Null)
        // Box::pin(async move { Ok(GraphQLValue::Null) })
    }

    pub async fn test_async1(&self) -> GraphQLResponse<GraphQLValue> {
        Ok(GraphQLValue::Int(1))
    }
}

async fn test_async() -> GraphQLResponse<GraphQLValue> {
    Ok(GraphQLValue::Null)
    // Box::pin(async move { Ok(GraphQLValue::Null) })
}

async fn test_async1() -> GraphQLResponse<GraphQLValue> {
    Ok(GraphQLValue::Int(1))
}

struct ResolverSt<'a> {
    pub resolve: GraphQLFuture<'a>,
}

// hashmapから取り出したresolverが&Pin型になっているので値が取り出せないが、async-graphqlではPinなのでjoin_allできている
// hashmapからgetした場合は所有権がhashmapにあるので、参照としてしか取り出すことができない
// removeすればhashmapから所有権がなくなるので、値として取得することができる
// hashmapにresolver関数を格納するのは無理そう
fn build_resolvers<'a>() -> HashMap<
    (&'a str, &'a str),
    Pin<Box<impl futures::Future<Output = GraphQLResponse<GraphQLValue>>>>,
> {
    let mut future_map = HashMap::new();
    future_map.insert(("query", "show"), Box::pin(test_async()));
    // future_map.insert(("query", "test"), Box::pin(test_async1()));
    future_map
}

#[cfg(test)]
mod tests {
    use super::test_async;
    use super::Query;
    use super::Resolver;
    use super::ResolverSt;

    #[tokio::test]
    async fn it_works() {
        let query = Query {};
        // traitに実装しているstructの参照を渡すことで値を得られる
        let value = Resolver::resolve(&query).await;
        println!("{:?}", value);
        println!("{:?}", &query.test_async().await);

        // assert_eq!(value, GraphQLValue::Null);
    }
}