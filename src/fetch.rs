use tauri_plugin_http::reqwest;

fn fetch(url){
let res = reqwest::get("http://my.api.host/data.json").await;
println!("{:?}", res.status()); // e.g. 200
println!("{:?}", res.text().await); // e.g Ok("{ Content }")
})
