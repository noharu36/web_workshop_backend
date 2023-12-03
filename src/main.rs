use axum::{
    http::{header::HeaderName, HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, post},
    //Json,
    Router,
};
use sns_backend::get_thread::get_threads_handler;
use sns_backend::login::auth;
use sns_backend::make_thread::create_thread;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
};
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let x_request_id = HeaderName::from_static("x-request-id");

    let app = Router::new()
        .route("/", get(get_threads_handler))
        .route("/login", post(auth))
        .route("/create", post(create_thread))
        //.route("/get", get(root))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MyRequestId::new(),
                ))
                .layer(PropagateRequestIdLayer::new(x_request_id))
                .layer(axum::middleware::from_fn(access_log_on_request)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//アクセスログの出力
#[derive(Clone)]
pub struct MyRequestId {}

impl MyRequestId {
    pub fn new() -> Self {
        MyRequestId {}
    }
}

impl MakeRequestId for MyRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().to_string();
        let request_id = HeaderValue::from_str(&uuid).unwrap();

        Some(RequestId::new(request_id))
    }
}

async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    info!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

//curlのコマンド curl localhost:8080/login -XPOST -H 'Content-Type: application/json' -d '{"name": "haru", "password": "pass"}'
//curlのコマンド curl localhost:8080/create -XPOST -H 'Content-Type: application/json' -d '{"title": "test-thread2", "body": "これはテスト", "user_id": "6551e079dc285b4db638b7ac", "user_name": "haru"}'
