// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use reqwest::Client;
use serde_json::Value;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn fetch_url() -> String {
    println!("Demo");
    let api_key = "sk-test-xxxx";
    let customer_id = "cust_demo_123";

    let url = format!(
        "https://api.sandbox.bridge.xyz/v0/customers/{}/hosted_tos",
        customer_id
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key)
        .send()
        .await
        .expect("request failed");

    let body: Value = res.json().await.expect("invalid json");
    println!("{:#?}", body);
    body["url"].as_str().unwrap().to_string()
}
