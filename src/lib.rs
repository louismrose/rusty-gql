mod container;
mod context;
mod error;
mod executor;
mod operation;
mod path;
mod request;
mod resolver;
mod server;
mod template;
mod types;
mod validation;

#[doc(hidden)]
pub use async_trait;

pub use context::{ExecutionContext, FieldContext, SelectionSetContext};
use error::GqlError;
pub use operation::OperationType;
pub use resolver::{Resolver, SelectionSetResolver};
pub use template::GraphiQLTemplate;
pub use types::schema::build_schema;
pub use types::{
    GqlArgument, GqlDirective, GqlEnum, GqlField, GqlInputObject, GqlInterface, GqlObject,
    GqlScalar, GqlTypeDefinition, GqlUnion, GqlValue, Schema,
};

pub type Response<T> = ::std::result::Result<T, GqlError>;

pub use rusty_gql_codegen::GqlModel;
pub use rusty_gql_codegen::Object;
