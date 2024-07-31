// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database_utils::{Type, WebInfo};

use crate::utils::set_window_shadow;
use crate::web::open;

mod database_utils;
mod db_operation;
mod utils;
mod web;

// 打开网站
#[tauri::command]
fn open_url(url: &str) {
    if let Err(e) = open(url) {
        eprintln!("Failed to open URL: {}", e);
    }
}

// 数据库操作
#[tauri::command]
fn set_type_info(type_name: &str) -> usize {
    db_operation::set_type_info(type_name)
}

#[tauri::command]
fn get_type_info_list() -> Vec<Type> {
    db_operation::get_type_info_list()
}

#[tauri::command]
fn get_type_info_by_id(id: u64) -> Type {
    db_operation::get_type_info_by_id(id)
}

#[tauri::command]
fn delete_type_info_by_id(id: u64) {
    db_operation::delete_type_info_by_id(id);
}

#[tauri::command]
fn update_type_info_by_id(id: u64, type_name: &str) {
    db_operation::update_type_info_by_id(id, type_name);
}

#[tauri::command]
fn get_web_info_list(name: &str, address: &str, type_name: &str) -> Vec<WebInfo> {
    db_operation::get_web_info_list(name, address, type_name)
}

#[tauri::command]
fn get_web_info_ofent_list(name: &str, address: &str, type_name: &str) -> Vec<WebInfo> {
    db_operation::get_web_info_ofent_list(name, address, type_name)
}

#[tauri::command]
fn delete_web_info_by_id(id: u64) {
    db_operation::delete_web_info_by_id(id);
}

// 数据库操作
#[tauri::command]
fn set_web_info(name: &str, address: &str, type_id: u64) -> usize {
    db_operation::set_web_info(name, address, type_id)
}

#[tauri::command]
fn get_web_info_by_id(id: u64) -> WebInfo {
    db_operation::get_web_info_by_id(id)
}

#[tauri::command]
fn update_web_info_by_id(id: u64, name: &str, address: &str, type_id: u64){
    db_operation::update_web_info_by_id(id, name, address, type_id);
}

#[tauri::command]
fn update_web_info_is_frequent_by_id(id: u64){
    db_operation::update_web_info_is_frequent_by_id(id);
}

#[tauri::command]
fn update_web_info_is_pinned_by_id(id: u64){
    db_operation::update_web_info_is_pinned_by_id(id);
}


fn main() {
    // 初始化链接，初始化表
    db_operation::create_table();
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_url,
            set_type_info,
            get_type_info_list,
            get_type_info_by_id,
            delete_type_info_by_id,
            update_type_info_by_id,
            get_web_info_list,
            delete_web_info_by_id,
            set_web_info,
            get_web_info_by_id,
            update_web_info_by_id,
            update_web_info_is_frequent_by_id,
            update_web_info_is_pinned_by_id,
            get_web_info_ofent_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
