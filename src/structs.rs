use reqwest::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct RequestData {
    pub rest_method: reqwest::Method,
    pub external_url: String,
    pub request_headers: HeaderMap,
    pub request_body: Option<String>,
    pub request_ttl: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AckResponse {
    status: String,
    status_code: u16,
    message: String,
    ack_id: String,
    request_data: RequestData,
}