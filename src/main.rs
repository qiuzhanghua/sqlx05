use dotenv::dotenv;
use std::env;

use sqlx::any::AnyPoolOptions;
use sqlx::{Connection, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let url = match env::var("TDF_URL") {
        Ok(x) => x,
        Err(_) => "sqlite://tdf.sqlite".to_owned(),
    };
    let pool = AnyPoolOptions::default().connect(&url).await?;
    let mut conn = pool.acquire().await?;
    conn.ping();
    // let x = sqlx::query("select version() v")
    //     .fetch_one(&mut conn)
    //     .await?;
    // let v: String = x.get(0);
    // println!("{:?}", v);
    conn.release();
    Ok(())
}
