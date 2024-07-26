use axum::{body::Body, extract::{Multipart, Request}, http::StatusCode, middleware::Next, response::IntoResponse, routing::post, Json, Router};
use http_body_util::BodyExt;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("frontend"))
        .route("/upload", post(upload))
        //
        //      #### Here ####
        //
        //
        // THE BUFFERING HERE SOLVES THE PROBLEM:
        // .layer(axum::middleware::from_fn(print_request_body))
}

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                let field_name = field.name().unwrap().to_string();

                match field_name.as_str() {
                    "image" => {
                        let image_bytes = field.bytes().await;
                        let Ok(image_bytes) = image_bytes else {
                            let status = image_bytes.unwrap_err().status();
                            return (status, Json(""));
                        };
                        let a = image_bytes.as_ptr();
                        println!("{a:?}");
                    }
                    _ => panic!(),
                }
            }
            // no more fields
            Ok(None) => break,
            Err(e) => {
                println!("status: {:?}", e.status());
                return (e.status(), Json(""))
            },
        }
    }
    
    (StatusCode::CREATED, Json(""))
}

async fn print_request_body(request: axum::extract::Request, next: Next) -> Result<impl IntoResponse, axum::response::Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

async fn buffer_request_body(request: axum::extract::Request) -> Result<axum::extract::Request, axum::response::Response> {
    let (parts, body) = request.into_parts();

    // this won't work if the body is a long running stream
    let bytes = body
        .collect() // <--- buffers all bytes, so the request is done from browser side?
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    Ok(Request::from_parts(parts, Body::from(bytes)))
}
