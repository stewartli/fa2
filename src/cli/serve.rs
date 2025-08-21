use std::path::PathBuf;
use serde::Deserialize;

use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use axum::{
    extract::Query, 
    http::StatusCode, 
    response::{Html, IntoResponse, Response}, 
    routing::get, 
    Router, 
};

#[derive(Deserialize)]
struct QmdReport {
    client: String,
    year: String, 
}

pub fn run(addr: &str){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("✓ server listening on {}", listener.local_addr().unwrap());

        let app = Router::new()
            .route("/", get(|| async { "Financial Audit Tool" }))
            // http://127.0.0.1:8090/report?client=beijing&year=2023
            .route("/report", get(show_report))
            // http://127.0.0.1:8090/temp/src/main.rs
            .nest_service("/temp", ServeDir::new("."))
            .fallback(handle_404);

        axum::serve(listener, app)
            .with_graceful_shutdown(stop_server())
            .await
            .unwrap();

        println!("✓ server sucessfully shutdown");
    })
}

async fn stop_server(){
    tokio::signal::ctrl_c().await.expect("fail Ctrl+C handler");
    println!("\n✓ server starting shutdown ...");
}

async fn show_report(Query(qmd): Query<QmdReport>) -> Response {
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_report = path_root
        .join("faproj/job")
        .join(qmd.client)
        .join(qmd.year)
        .join("report/report.html");
    // consider stream
    if path_report.exists(){
        let res = std::fs::read_to_string(path_report).expect("fail to read path report");
        (StatusCode::OK, Html(res)).into_response()
    }else{
        (StatusCode::BAD_REQUEST, Html("nothing")).into_response()
    }
}

async fn handle_404() -> impl IntoResponse {
    "no response for you"
}

