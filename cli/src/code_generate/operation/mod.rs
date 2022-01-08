mod field_file;
mod operation_mod_file;

use futures_util::future::try_join_all;
use rusty_gql::{self, GqlField, OperationType};
use std::{collections::BTreeMap, io::Error};

use self::{field_file::FieldFile, operation_mod_file::OperationModFile};

use super::{create_file, path_str};

pub async fn create_operation_files(
    operations: &BTreeMap<String, GqlField>,
    operation_type: OperationType,
    base_path: &str,
    interface_names: &Vec<String>,
) -> Result<Vec<()>, Error> {
    let mut futures = Vec::new();

    for (_, field) in operations.iter() {
        let filename = path_str(
            vec![
                base_path,
                &operation_type.to_string().to_lowercase(),
                &field.name,
            ],
            true,
        );
        let task = create_file(FieldFile {
            file_name: field.name.to_string(),
            def: field,
            path: filename,
            interface_names: &interface_names,
        });
        futures.push(task);
    }

    create_file(OperationModFile {
        operation_type: operation_type.clone(),
        operations,
        path: path_str(
            vec![base_path, &operation_type.to_string().to_lowercase(), "mod"],
            true,
        ),
        interface_names: &interface_names,
    })
    .await?;

    try_join_all(futures).await
}
