use std::fmt::format;

use axum::{
	Router, extract::Request, http::StatusCode, response::IntoResponse, routing::get
};

async fn greet() -> &'static str {
	"Hello!!!"
}

async fn hc(req: Request) -> impl IntoResponse {
	StatusCode::OK
}

#[tokio::main]
async fn main() {
	let app = Router::new()
			.route("/", get(greet))
			.route("/hc",get(hc));

	let lis = tokio::net::TcpListener::bind("127.0.0.1:3000")
			.await
			.unwrap();

	println!("listening on {}", lis.local_addr().unwrap());

	axum::serve(lis, app).await.unwrap();
}
