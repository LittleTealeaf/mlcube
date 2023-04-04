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

    // demo_fetch_models(&connection).await;
    demo_select_with_binds(&connection, 3).await;
}

async fn demo_fetch_models(connection: &Pool<Mssql>) {
    let query: Vec<Model> = sqlx::query_as("SELECT * FROM Models")
        .fetch_all(connection)
        .await
        .unwrap();

    for item in query.into_iter() {
        println!("{:?}", item);
    }
}

#[derive(FromRow, Debug)]
struct Epoch {
    Epoch: i32,
    Loss: Option<f32>,
    Reward: Option<f32>,
}

async fn demo_select_with_binds(connection: &Pool<Mssql>, ModelId: i32) {
    let query: Vec<Epoch> =
        sqlx::query_as("SELECT Epoch, Loss, Reward FROM Epochs WHERE ModelId = @1")
            .bind(ModelId)
            .fetch_all(connection)
            .await
            .unwrap();

    for item in query {
        println!("{:?}", item);
    }
}
