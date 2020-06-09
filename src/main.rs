use dotenv;
use serde_json::json;
use sqlx::PgPool;
use sqlx::Pool;
use thiserror::Error;

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
        let json = json!({"array":     [1,n,3], "val": {"test": 1}});
        Ok(tide::Response::new(tide::StatusCode::Ok).body_json(&json)?)
    });

    app.listen("127.0.0.1:8080").await.unwrap();
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
    use http_service::{HttpService, Request, Response};

    #[async_std::test]
    async fn a_test() {}
}
