use hyper::{ Body, Request, Response, Server, StatusCode };
use hyper::service::{ make_service_fn, service_fn };
use std::convert::Infallible;
use std::net::SocketAddr;
use serde_json::json;
use reqwest::Client;

mod utils;
mod structs;

pub async fn make_external_request(
    request_data: structs::RequestData
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let timeout = std::time::Duration::from_secs(
        std::cmp::min(request_data.request_ttl.unwrap_or(165), 165)
    );

    let response = client
        .request(request_data.rest_method.clone(), &request_data.external_url)
        .headers(request_data.request_headers.clone())
        .body(request_data.request_body.unwrap_or_default())
        .timeout(timeout)
        .send().await?;

    let response_text = response.text().await?;
    Ok(response_text)
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut data = Vec::new();

    while let Some(chunk) = req.data().await {
        data.extend_from_slice(&chunk);
    }
    let requestData: serde_json::Value = serde_json::from_slice(&data).unwrap();

    let ack_id = utils::get_ack_id();

    let ack_response =
        json!({
        "status": "success",
        "statusCode": 201,
        "message": "Request received and ACK sent",
        "ackId": ack_id.to_string(),
        "requestData": requestData,
    });

    let response = Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&ack_response).unwrap()))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}