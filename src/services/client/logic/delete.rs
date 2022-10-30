use crate::{cores::error::service::Error, services::client::repo::db::DbRepo, utils::validation};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &str) -> Result<String, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.delete(code).await
}

fn validate(req: &str) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if req.is_empty() {
        validation.add_str("Code is empty");
    }

    validation.check()
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         cores::error::service::Error,
//         services::state::{logic::delete::execute, repo::db::MockDbRepo},
//     };
//     use mockall::predicate::eq;
//     use std::sync::Arc;

//     #[tokio::test]
//     async fn fail_validate_code_empty() -> Result<(), Error> {
//         let mock_db_repo = MockDbRepo::new();

//         let req = String::from("");
//         let res = execute(Arc::new(mock_db_repo), &req).await;

//         assert!(res.is_err());
//         assert_eq!(
//             Error::BadRequest("Code is empty".to_owned()),
//             res.unwrap_err()
//         );

//         Ok(())
//     }

//     #[tokio::test]
//     async fn success() -> Result<(), Error> {
//         let req = "TEST";

//         let mut mock_db_repo = MockDbRepo::new();
//         mock_db_repo
//             .expect_delete()
//             .with(eq(req.clone()))
//             .once()
//             .returning(move |req| {
//                 let cloned_req = req.to_string();
//                 Box::pin(async { Ok(cloned_req) })
//             });

//         let res = execute(Arc::new(mock_db_repo), &req).await;

//         let return_result = res?;
//         assert_eq!(return_result, req);
//         Ok(())
//     }
// }
