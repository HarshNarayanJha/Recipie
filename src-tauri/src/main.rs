// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use category::{Catagories, Category};
use recipe::Recipe;
use reqwest::Client;

mod recipe;
mod category;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_all_categories() -> Catagories {
    let url = String::from("https://www.themealdb.com/api/json/v1/1/categories.php");

    let request = Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Catagories>()
        .await
        .unwrap();

    request
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_categories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
