use std::{ convert::Infallible, net::SocketAddr };
use hyper::service::{ make_service_fn, service_fn };
use hyper::{ Body, Client, Request, Response, Server };
use hyper::Method;
use tokio::time::{ Duration };
use serde::{ Deserialize };
use reqwest;

use http::header::{ HeaderName, HeaderValue };

type HttpClient = Client<hyper::client::HttpConnector>;

#[derive(Debug, Deserialize)]
struct RequestData {
    externalUrl: String,
    restMethod: String,
    requestHeaders: serde_json::Value,
    requestBody: serde_json::Value,
    requestTtl: u64,
    responseCallBack: String,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8100));
    let client = Client::new();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| process_request(client.clone(), req)))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn process_request(
    client: HttpClient,
    req: Request<Body>
) -> Result<Response<Body>, hyper::Error> {
    let req_bytes = hyper::body::to_bytes(req.into_body()).await?;


    let req_string = String::from_utf8(req_bytes.to_vec()).unwrap();

    let request_data: Result<RequestData, serde_json::Error> = serde_json::from_str(&req_string);

    let request_data = match request_data {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error parsing request data: {:?}", err);
            return Ok(Response::builder().status(400).body(Body::empty()).unwrap());
        }
    };

    let external_url = request_data.externalUrl;
    let method = match request_data.restMethod.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        _ => {
            eprintln!("Invalid method: {}", request_data.restMethod);
            return Ok(Response::builder().status(400).body(Body::empty()).unwrap());
        }
    };
    let headers = request_data.requestHeaders;
    let request_body = request_data.requestBody;
    let max_timeout = Duration::from_secs(std::cmp::min(165, request_data.requestTtl));
    let callback_url = request_data.responseCallBack;

    let api_request = Request::builder()
        .method(method)
        .uri(external_url)
        .body(Body::from(request_body.to_string()));

    let response = client.request(api_request).await?;

    if response.status().is_success() {
        let response_body = response.body();
        let response_str = response_body.as_str().unwrap();
        let response_callback_url = request_data.responseCallBack;
        let response_client = reqwest::Client::new();
        let res = client::post(response_callback_url).body(response_str).send().await?;
        if !res.status().is_success() {
            eprintln!("Failed to post response to responseCallBack endpoint: {}", res.status());
        }
        println!("Request successful!");
    } else {
        // The request was not successful
        println!("Request failed with status code: {}", response.status());
    }
}