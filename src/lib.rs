use tauri::http::HeaderMap;
use tauri_plugin_http::reqwest;

enum HTTPMethod {
    PUT,
    POST,
    GET,
    DELETE
}

fn fetch(httpmethod: HTTPMethod, url: &str, headers: Option<HeaderMap>){
    
}

 async fn get(url: &str) -> Result<Response> {
    let res =  reqwest::get(url)
    .await?;

    match res.status()
}

