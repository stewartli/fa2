use std::path::PathBuf;
use serde::Deserialize;

use axum::{extract::Query, http::StatusCode, response::{Html, IntoResponse, Response}, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct QmdReport {
    client: String,
    year: String, 
}

pub fn run(addr: &str){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("server listening on {}", listener.local_addr().unwrap());

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/report", get(show_report))
            // /asset/index.html
            .nest_service("/asset", ServeDir::new("asset"));

        axum::serve(listener, app).await.unwrap();
    })
}

async fn show_report(Query(qmd): Query<QmdReport>) -> Response {
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_report = path_root
        .join("faproj/job")
        .join(qmd.client)
        .join(qmd.year)
        .join("report/report.html");
    // http://127.0.0.1:8090/report?client=beijing&year=2023
    if path_report.exists(){
        let res = std::fs::read_to_string(path_report).unwrap();
        (StatusCode::OK, Html(res)).into_response()
    }else{
        (StatusCode::BAD_REQUEST, Html("nothing")).into_response()
    }
}

