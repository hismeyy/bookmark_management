use rusqlite::Connection;
use std::sync::{Arc, Mutex, OnceLock};

use crate::database_utils::{self, Type, WebInfo};

// 设置全局变量
const TABLE_NAME: &str = "D:/Project/rust/hamster/web_data_db.db";
// const TABLE_NAME: &str = "./web_data_db.db";

// 设置全局链接数据库
#[derive(Debug)]
struct ConnectionInfo {
    connection: Mutex<Connection>,
}

static CONNECTION_INFO: OnceLock<Arc<ConnectionInfo>> = OnceLock::new();

impl ConnectionInfo {
    fn global() -> &'static Arc<ConnectionInfo> {
        // 获取或初始化数据库连接
        CONNECTION_INFO.get_or_init(|| {
            let conn = database_utils::connection_database(TABLE_NAME)
                .expect("连接数据库时发生未知错误！");
            Arc::new(ConnectionInfo {
                connection: Mutex::new(conn),
            })
        })
    }
}

// 创建所需要的表
pub fn create_table() {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    let _ = database_utils::create_type_info_table(conn);
    let _ = database_utils::create_web_info_table(conn);
}

// 设置type_info数据
pub fn set_type_info(type_name: &str) -> usize {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::install_type_info(conn, type_name)
}

// 查询type_info列表
pub fn get_type_info_list() -> Vec<Type> {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    match database_utils::select_type_info(conn) {
        Ok(type_info_list) => type_info_list,
        Err(_) => {
            let result: Vec<Type> = Vec::new();
            result
        }
    }
}

// 查询type_info通过id
pub fn get_type_info_by_id(id: u64) -> Type {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    let type_info = database_utils::select_type_info_by_id(conn, id).unwrap();
    type_info
}

// 删除type_info通过id
pub fn delete_type_info_by_id(id: u64) {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::delete_type_info_by_id(conn, id).unwrap();
}

// 修改type_info通过id
pub fn update_type_info_by_id(id: u64, type_name: &str){
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::update_type_info_by_id(conn, id, type_name).unwrap();
}

// =============================================================================================== web_info
// 查询web_info列表
pub fn get_web_info_list(name: &str, address: &str, type_name: &str) -> Vec<WebInfo> {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    match database_utils::select_web_info(conn, name, address, type_name) {
        Ok(web_info_list) => {
            println!("{:?}", web_info_list);
            web_info_list
        },
        Err(_) => {
            let result: Vec<WebInfo> = Vec::new();
            result
        }
    }
}

pub fn get_web_info_ofent_list(name: &str, address: &str, type_name: &str) -> Vec<WebInfo> {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    match database_utils::get_web_info_ofent_list(conn, name, address, type_name) {
        Ok(web_info_list) => web_info_list,
        Err(_) => {
            let result: Vec<WebInfo> = Vec::new();
            result
        }
    }
}

// 删除web_info通过id
pub fn delete_web_info_by_id(id: u64) {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::delete_web_info_by_id(conn, id).unwrap();
}

// 设置web_info数据
pub fn set_web_info(name: &str, address: &str, type_id: u64) -> usize {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::install_web_info(conn, name, address, type_id)
}

// 查询web_info通过id
pub fn get_web_info_by_id(id: u64) -> WebInfo {
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    let web_info = database_utils::select_web_info_by_id(conn, id).unwrap();
    web_info
}

// 修改web_info通过id
pub fn update_web_info_by_id(id: u64, name: &str, address: &str, type_id: u64){
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();
    database_utils::update_web_info_by_id(conn, id, name, address, type_id).unwrap();
}

// 设置常用通过id
pub fn update_web_info_is_frequent_by_id(id: u64){
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();

    let web_info: WebInfo = database_utils::select_web_info_by_id(conn, id).unwrap();
    
    let mut is_frequent_temp = true;
    if web_info.is_frequent() {
        is_frequent_temp = false;
    }

    database_utils::update_web_info_is_frequent_by_id(conn, id, is_frequent_temp).unwrap();
}

// 设置置顶通过id
pub fn update_web_info_is_pinned_by_id(id: u64){
    let connection_info = ConnectionInfo::global();
    let conn = &connection_info.connection.lock().unwrap();

    let web_info: WebInfo = database_utils::select_web_info_by_id(conn, id).unwrap();
    
    let mut is_pinned_temp = true;
    if web_info.is_pinned() {
        is_pinned_temp = false;
    }

    database_utils::update_web_info_is_pinned_by_id(conn, id, is_pinned_temp).unwrap();
}
