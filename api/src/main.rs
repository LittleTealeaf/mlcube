use std::env;

use dotenvy::dotenv;
use sqlx::{
    mssql::{Mssql, MssqlPool},
    FromRow, Pool,
};

#[derive(FromRow, Debug)]
struct Model {
    ModelId: i32,
    ModelName: String,
    GitHash: Option<String>,
}

#[actix_web::main]
async fn main() {
    dotenv().unwrap();

    let connection = MssqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let values = get_epochs_for_model(&connection, 1).await;
    println!("{:?}", values.len());
}

#[derive(FromRow, Debug)]
struct Epoch {
    epoch: i32,
    loss: Option<f64>,
    reward: Option<f64>,
}

async fn get_epochs_for_model(connection: &Pool<Mssql>, model_id: i32) -> Vec<Epoch> {
    sqlx::query_as("SELECT epoch, loss, reward FROM Epochs WHERE ModelId = $1")
        .bind(model_id)
        .fetch_all(connection)
        .await
        .unwrap()
}
