use dotenv;
use serde_json::json;
use sqlx::PgPool;
use sqlx::Pool;
use thiserror::Error;
use tide::Server;

// fn min<T: Ord>(a: T, b: T) -> T {
//     let r = Range {
//         start: 100,
//         end: 200,
//     };
//     let ret = if a < b { a } else { b };
//     ret
// }

// struct Range<T> {
//     start: T,
//     end: T,
// }

#[async_std::main]
async fn main() {
    let app = server().await;

    let r = app.listen("127.0.0.1:8080").await.unwrap();
    return r;
}

async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    println!("{}", database_url);

    let db_pool: PgPool = Pool::new(&database_url).await.unwrap();

    let mut app = tide::Server::with_state(State { db_pool });

    // let account = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
    // .fetch_one(&mut conn)
    // .await?;

    // anonymous struct has `#[derive(Debug)]` for convenience
    // println!("{:?}", account);
    // println!("{}: {}", account.id, account.name);

    app.at("/").get(|req: tide::Request<State>| async move {
        let db_pool_2 = &req.state().db_pool;
        // let sql = "select 1 as one where 1 = 2";
        let rows = sqlx::query!("select now() as now")
            .fetch_one(db_pool_2)
            .await?;
        //  dbg!(rows);

        println!("{:?}", rows);
        println!("{:?}", rows.now);
        Ok("Hello from uptime-rs.")
    });

    app.at("/json").get(|_| async move {
        let n = 100;
        let json = json!({"array": [1,n,3], "value": {"test": 1}});
        Ok(tide::Response::new(tide::StatusCode::Ok).body_json(&json)?)
    });
    app
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}

#[derive(Error, Debug)]
enum ApplicationError {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    JSonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use futures::{executor::block_on, prelude::*};
    use http_service::{HttpService, Response};
    use http_types::{Method, Request, Url};

    #[derive(Debug)]
    pub struct TestBackend<T: HttpService> {
        service: T,
        connection: T::Connection,
    }

    impl<T: HttpService> TestBackend<T> {
        fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
            let connection = block_on(service.connect().into_future())?;
            Ok(Self {
                service,
                connection,
            })
        }

        // Send a request
        pub fn simulate(
            &mut self,
            req: Request,
        ) -> Result<Response, <T::ResponseFuture as TryFuture>::Error> {
            block_on(
                self.service
                    .respond(self.connection.clone(), req)
                    .into_future(),
            )
        }
    }
    pub fn make_server<T: HttpService>(
        service: T,
    ) -> std::result::Result<test::TestBackend<T>, <T as http_service::HttpService>::ConnectionError>
    {
        TestBackend::wrap(service)
    }

    #[async_std::test]
    async fn tests() {
        let server = server().await;
        let mut server = make_server(server).unwrap();
        let url = Url::parse("http://localhost/json").unwrap();
        dbg!(url.clone());
        // println!("{:?}", url);
        let req = Request::new(Method::Get, url);
        let res = server.simulate(req).unwrap();
        assert_eq!(res.status(), 200);
        let z = res.body_string().await?;
        assert_eq!(z, "{\"array\":[1,100,3],\"value\":{\"test\":1}}");
    }
}
