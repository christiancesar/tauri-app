// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mongodb::{bson::{Document,doc}, Collection};
use futures::stream::TryStreamExt;
mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet() -> Result<Document, String> {
    let client = db::db_connection().await.map_err(|_| "Erro ao conectar ao banco de dados".to_string())?;
    let customers_collection: Collection<Document> = client.database("hunter-module-app").collection("customers");

    let customer = customers_collection
    .find_one(
        doc! {
            "name": "NATHALIA COIMBRA DO AMARAL"
        },
    )
    .await
    .map_err(|_| "Erro ao buscar o cliente".to_string())?;


    match customer {
        Some(document) => Ok(document), // Retorna o documento se encontrado
        None => Err("Cliente não encontrado".to_string()), // Erro se nenhum documento for encontrado
    }
}

#[tokio::main]
async fn main() {
    let client = db::db_connection().await.unwrap();
    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}   
