mod container;
mod error;
mod executor;
mod graphql_object;
mod graphql_value;
mod operation;
mod resolver;
mod server;
mod template;
mod types;
mod test_async;

use error::GraphQLError;
pub use resolver::Resolver;
pub use template::GraphiQLTemplate;
pub use types::GraphQLSchema;

pub type GraphQLResponse<T> = ::std::result::Result<T, GraphQLError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
