use crate::cores::error::service::Error;
use crate::services::state::model::entity::State;
use crate::services::state::repo::db::DbRepo;
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>) -> Result<Vec<State>, Error> {
    tracing::debug!("executing ...");
    repo.get_all().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cores::database::pg::db_time_now,
        services::state::{model::entity::State, repo::db::MockDbRepo},
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn success() -> Result<(), Error> {
        let mut mock_db_repo = MockDbRepo::new();
        mock_db_repo.expect_get_all().once().returning(move || {
            Box::pin(async {
                Ok(vec![State {
                    id: 1,
                    code: String::from("TEST"),
                    description: Some(String::from("test")),
                    webhooks: Some(vec![String::from("test_app")]),
                    create_time: db_time_now(),
                    update_time: db_time_now(),
                }])
            })
        });

        let res = execute(Arc::new(mock_db_repo)).await;

        let return_result = res?.clone();
        assert_eq!(return_result.len(), 1);
        assert_eq!(return_result[0].id, 1);
        assert_eq!(return_result[0].code, "TEST");
        assert_eq!(return_result[0].description, Some(String::from("test")));
        assert_eq!(return_result[0].webhooks.as_ref().unwrap().len(), 1);
        assert_eq!(
            return_result[0].webhooks.as_ref().unwrap()[0],
            String::from("test_app")
        );
        assert_ne!(return_result[0].create_time, chrono::NaiveDateTime::MIN);
        assert_ne!(return_result[0].update_time, chrono::NaiveDateTime::MIN);
        Ok(())
    }
}
