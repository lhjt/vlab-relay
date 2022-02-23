use mongodb::{
    bson::doc,
    options::{ClientOptions, UpdateOptions},
    Client,
};
use serde::{Deserialize, Serialize};
use snafu::{whatever, Whatever};
use tracing::{error, instrument};

/// Represents a student who has access to use the current relay server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) zid:   String,
    pub(crate) token: String,
}

#[derive(Debug, Clone)]
pub(crate) struct UserManager {
    db_client: mongodb::Client,
}

impl UserManager {
    #[instrument]
    pub(crate) async fn new() -> Self {
        let client_options = match ClientOptions::parse(
            std::env::var("MONGODB_URI").expect("no MONGODB_URI was provided"),
        )
        .await
        {
            Ok(options) => options,
            Err(e) => {
                error!(
                    "Failed to create parse mongodb client options. Is your string valid? {}",
                    e
                );
                panic!()
            },
        };

        let db_client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to create mongodb client. {}", e);
                panic!()
            },
        };

        Self { db_client }
    }

    fn get_users_collection(&self) -> mongodb::Collection<User> {
        self.db_client.database("relay").collection("users")
    }

    #[instrument]
    pub(crate) async fn get_by_zid(&self, zid: &str) -> Option<User> {
        let filter = doc! {"zid": zid};
        let user = self.get_users_collection().find_one(filter, None).await;

        match user {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(e) => {
                error!("[get_by_zid] error: {}", e);
                None
            },
        }
    }

    #[instrument]
    pub(crate) async fn get_by_token(&self, token: &str) -> Option<User> {
        let filter = doc! {"token": token};
        let user = self.get_users_collection().find_one(filter, None).await;

        match user {
            Ok(Some(user)) => Some(user),
            Ok(None) => None,
            Err(e) => {
                error!("[get_by_token] error: {}", e);
                None
            },
        }
    }

    #[instrument]
    pub(crate) async fn upsert_user(&self, user: User) -> Result<(), Whatever> {
        let collection = self.get_users_collection();
        let result = collection
            .update_one(
                doc! {"zid": user.zid},
                doc! {"token": user.token},
                Some(UpdateOptions::builder().upsert(true).build()),
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("[upsert_user] error: {}", e);
                whatever!("failed to upsert: {}", e)
            },
        }
    }

    #[instrument]
    pub(crate) async fn delete_by_zid(&self, zid: &str) -> Result<(), Whatever> {
        let collection = self.get_users_collection();
        let result = collection.delete_one(doc! {"zid": zid}, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("[delete_by_zid] error: {}", e);
                whatever!("failed to delete: {}", e)
            },
        }
    }
}
