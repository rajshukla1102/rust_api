use axum::{
    extract::Extension,
    routing::get,
    Router};

use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod controllers;
mod models;

#[tokio::main]
async fn main(){
    dotenv().ok();
    let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to make connection");

    let app = Router::new()
        .route("/get_user",get(controllers::auth::get_user))
        .route("/user_data_json_array",get(controllers::auth::get_user_data_json_array))
        .layer(Extension(pool));
    
    

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([127,0,0,1],3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}