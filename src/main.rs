use actix_web::{ web, App, HttpResponse, HttpServer, Result };
use serde::{ Deserialize, Serialize };
use std::time::Duration;
use reqwest::Method;
use reqwest;
use serde_json::from_str;

#[allow(dead_code)]
#[allow(unused_variables)]

// #[derive(Debug, Deserialize)]
// pub struct RequestData {
//     #[serde(rename = "external_url")]
//     pub external_url: String,
//     #[serde(rename = "rest_method")]
//     pub rest_method: String,
//     #[serde(rename = "request_headers")]
//     pub request_headers: String,
//     #[serde(rename = "request_body:")]
//     pub request_body: String,
//     #[serde(rename = "request_ttl")]
//     pub request_ttl: u64,
//     #[serde(rename = "response_call_back")]
//     pub response_call_back: String,
// }
#[derive(Debug, Deserialize)]
struct RequestData {
    external_url: String,
    rest_method: String,
    request_headers: String,
    request_body: String,
    request_ttl: u64,
    response_call_back: String,
}

async fn process_request(data: web::Json<RequestData>) -> Result<HttpResponse> {
    let external_url = &data.external_url;
    let rest_method = match data.rest_method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        &_ => Method::GET,
    };
    // let request_headers = &data.request_headers;
    let request_body: serde_json::Value = from_str(&data.request_body).unwrap();
    let request_ttl = data.request_ttl.min(165); // Set your max timeout
    let response_call_back = &data.response_call_back;

    let client = reqwest::Client::builder().build();

    match client {
        Ok(client) => {
            let api_request_builder = client.request(rest_method, external_url);

            let json_data = serde_json::to_vec(&request_body)?;

            let json_string = String::from_utf8_lossy(&json_data);
            println!("JSON data: {}", json_string);

            let resp = api_request_builder
                .timeout(Duration::from_secs(request_ttl))
                .header("Content-Type", "application/json")
                .body(json_data)
                .send().await;

            match resp {
                Ok(resp) => {
                    let response = resp.text().await.unwrap();
                    println!("Response: {:?}", response);
                    Ok(HttpResponse::Ok().json(response))
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    Ok(HttpResponse::InternalServerError().json(format!("{:?}", e)))
                }
            }

            // Implement your logic to send the API response to the callback URL
            // ...
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(format!("{:?}", err)))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().route("/", web::post().to(process_request)) })
        .bind("127.0.0.1:8100")?
        .run().await
}
