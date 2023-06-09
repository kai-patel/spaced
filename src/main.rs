use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use database::{Database, DatabaseHandle, DatabaseTrait};
use dotenvy::dotenv;
use models::DbUser;
use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

use axum::{self, routing};

mod database;
mod follow;
mod http;
mod models;
mod person;
mod schema;

fn app<T, U, V>(config: FederationConfig<T>) -> axum::Router
where
    T: DatabaseTrait<U, V> + std::clone::Clone + std::marker::Sync + std::marker::Send + 'static,
{
    axum::Router::new()
        .route("/:name", routing::get(http::http_get_user))
        .route("/.well-known/webfinger", routing::get(http::webfinger))
        .route("/ping", routing::get(http::http_ping))
        .layer(FederationMiddleware::new(config))
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("spaced=debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database: Database = Database::new();
    let db_handler: DatabaseHandle<Database> = DatabaseHandle::new(database);

    let config = FederationConfig::builder()
        .domain("0.0.0.0")
        .app_data(db_handler)
        .debug(true)
        .build()?;

    let app = app::<DatabaseHandle<Database>, DbUser, diesel::result::Error>(config);

    tracing::debug!("app created");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    tracing::debug!("listening on {}:{}", addr.ip(), addr.port());

    server.await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use tower_http::trace::TraceLayer;

    use crate::database::DbHandler;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;

    struct DB {
        pub db_conn: Mutex<Box<Vec<DbUser>>>,
    }

    impl DbHandler<Box<Vec<DbUser>>> for DatabaseHandle<DB> {
        fn db_conn(&self) -> &Mutex<Box<Vec<DbUser>>> {
            &self.0.db_conn
        }
    }

    impl Clone for DatabaseHandle<DB> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl DatabaseTrait<DbUser, anyhow::Error> for DatabaseHandle<DB> {
        fn read_local_user(&self, query: &str) -> Result<DbUser, anyhow::Error> {
            let lock = self.db_conn().lock().unwrap();

            for user in lock.iter() {
                if user.name == query && user.local {
                    return Ok(user.clone());
                }
            }

            Err(anyhow::Error::msg("Could not find user"))
        }

        fn read_from_id(&self, query: &str) -> Result<DbUser, anyhow::Error> {
            let lock = self.db_conn().lock().unwrap();

            for user in lock.iter() {
                if user.id == query {
                    return Ok(user.clone());
                }
            }

            Err(anyhow::Error::msg("Could not find user"))
        }

        fn from_json(&self, input: &DbUser) -> Result<usize, anyhow::Error> {
            let mut lock = self.db_conn().lock().unwrap();

            let mut total = 0;
            let new = input.clone();

            for i in 0..lock.len() {
                let user = &lock[i];
                if user.id == input.id {
                    lock[i] = DbUser { ..new.clone() };
                    total += 1;
                }
            }

            if total > 0 {
                Ok(total)
            } else {
                Err(anyhow::Error::msg("Error"))
            }
        }
    }

    #[actix_rt::test]
    async fn get_absent_user() {
        let database: DB = DB {
            db_conn: Default::default(),
        };

        let db_handler: DatabaseHandle<DB> = DatabaseHandle::new(database);

        let config = FederationConfig::builder()
            .domain("localhost")
            .app_data(db_handler)
            .debug(true)
            .build()
            .unwrap();

        let app = app::<DatabaseHandle<DB>, DbUser, anyhow::Error>(config)
            .layer(TraceLayer::new_for_http());

        let response = app
            .oneshot(
                Request::builder()
                    .header("Accept", "application/activity+json")
                    .uri("/fakeuser/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("body: {:?}", body);
        assert_eq!(body, hyper::body::Bytes::from(""));
    }

    #[actix_rt::test]
    async fn absent_webfinger() {
        let database: DB = DB {
            db_conn: Default::default(),
        };

        let db_handler: DatabaseHandle<DB> = DatabaseHandle::new(database);

        let config = FederationConfig::builder()
            .domain("localhost")
            .app_data(db_handler)
            .debug(true)
            .build()
            .unwrap();

        let app = app::<DatabaseHandle<DB>, DbUser, anyhow::Error>(config)
            .layer(TraceLayer::new_for_http());

        let response = app
            .oneshot(
                Request::builder()
                    .header("accept", "*/*")
                    .uri("/.well-known/webfinger/?resource=acct:LemmyDev@mastodon.social")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("body: {:?}", body);
        assert_eq!(body, hyper::body::Bytes::from(""));
    }

    #[actix_rt::test]
    async fn ping() {
        let database: DB = DB {
            db_conn: Default::default(),
        };

        let db_handler: DatabaseHandle<DB> = DatabaseHandle::new(database);

        let config = FederationConfig::builder()
            .domain("localhost")
            .app_data(db_handler)
            .debug(true)
            .build()
            .unwrap();

        let app = app::<DatabaseHandle<DB>, DbUser, anyhow::Error>(config)
            .layer(TraceLayer::new_for_http());

        let response = app
            .oneshot(
                Request::builder()
                    .header("accept", "*/*")
                    .uri("/ping")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("body: {:?}", body);
        assert_eq!(body, hyper::body::Bytes::from("pong"));
    }
}
