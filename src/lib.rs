use reqwest::Response;
use tauri::http::HeaderMap;
use tauri_plugin_http::reqwest;
use tauri_plugin_log::log::{error, info, warn};

enum HTTPMethod {
    PUT,
    POST,
    GET,
    DELETE,
}

async fn fetch(
    httpmethod: HTTPMethod,
    url: &str,
    headers: Option<HeaderMap>,
) -> Result<Response, Box<dyn std::error::Error>> {
    match httpmethod {
        HTTPMethod::GET => return get(url).await,
        HTTPMethod::PUT => todo!(),
        HTTPMethod::POST => todo!(),
        HTTPMethod::DELETE => todo!(),
    }
}

async fn get(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let res = match reqwest::get(url).await {
        Ok(response) => response,
        Err(err) => {
            error!("GET request failed: {url}. Error: {}", err);
            return Err(Box::new(err));
        }
    };

    let res_status = res.status();
    if res_status.is_success() || res_status.is_informational() {
        info!("GET({}): {}", res_status, url);
    }

    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "ok"}"#)
            .create_async()
            .await;

        let url = format!("{}/test", server.url());
        let result = get(&url).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_404_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/notfound")
            .with_status(404)
            .with_body("Not Found")
            .create_async()
            .await;

        let url = format!("{}/notfound", server.url());
        let result = get(&url).await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_network_error() {
        let result = get("http://localhost:1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "TODO"]
    async fn test_get_logs_success() {}

    #[tokio::test]
    async fn test_get_invalid_url() {
        let result = get("not-a-valid-url").await;
        assert!(result.is_err());
    }
}
