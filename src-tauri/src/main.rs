#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod notification_model;
use reqwest::{self, Error};
use std::env;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_github_notification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_github_notification() -> Option<Vec<notification_model::model::Notification>> {
    let data = match get_github_notifications_data().await {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    let notifications: Vec<notification_model::model::Notification> =
        serde_json::from_str(&data).unwrap();
    Some(notifications)
}

async fn get_github_notifications_data() -> Result<String, Error> {
    // Get ENV
    let mut username: String = String::new();
    let mut token: String = String::new();
    match env::var("GITHUB_USERNAME") {
        Ok(val) => username = val,
        Err(e) => println!("GITHUB_USERNAME not found: {}", e),
    }
    match env::var("GITHUB_TOKEN") {
        Ok(val) => token = val,
        Err(e) => println!("GITHUB_TOKEN not found: {}", e),
    }

    let client = reqwest::Client::new();
    let res = match client
        .get("https://api.github.com/notifications")
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", username)
        .header("Authorization", "Bearer ".to_owned() + token.as_str())
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    match res.text().await {
        Ok(body) => Ok(body),
        Err(e) => {
            return Err(e);
        }
    }
}
