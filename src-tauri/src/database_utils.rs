use rusqlite::{params, Connection, Error, Result};
use serde::Serialize;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Serialize)]
pub struct Type {
    id: i32,
    type_name: String,
}

// 连接数据库
#[allow(dead_code)]
pub fn connection_database(connection_db: &str) -> Result<Connection> {
    println!("正在连接数据库...");
    match Connection::open(connection_db) {
        Ok(conn) => {
            println!("连接成功!");
            Ok(conn)
        }
        Err(e) => {
            eprintln!("连接失败!{}", e);
            Err(e)
        }
    }
}

// =========================================type_info========================
// 创建type_info表
#[allow(dead_code)]
pub fn create_type_info_table(conn: &Connection) -> Result<bool> {
    match conn.execute(
        "
                CREATE TABLE IF NOT EXISTS type_info (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    type_name TEXT NOT NULL
                );
        ",
        params![],
    ) {
        Ok(_) => {
            println!("type_info表新增成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("type_info表新增失败!{}", e);
            Ok(false)
        }
    }
}

// 新增数据到type_info
#[allow(dead_code)]
pub fn install_type_info(conn: &Connection, type_name: &str) -> usize {
    match conn.execute(
        "
        INSERT INTO type_info (type_name) VALUES (?);
    ",
        params![type_name],
    ) {
        Ok(_) => {
            println!("新增成功!");
            let mut stmt = conn.prepare(" SELECT last_insert_rowid();").unwrap();

            let mut rows = stmt.query(params![]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let id: usize = row.get(0).unwrap();
                id
            } else {
                0
            }
        }
        Err(e) => {
            eprintln!("新增失败!{}", e);
            0
        }
    }
}

// 修改type_info中的数据通过id
#[allow(dead_code)]
pub fn update_type_info_by_id(conn: &Connection, id: u64, type_name: &str) -> Result<bool> {
    match conn.execute(
        "
        UPDATE type_info SET type_name = ? WHERE id = ?;
    ",
        params![type_name, id],
    ) {
        Ok(_) => {
            println!("更新成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("更新失败!{}", e);
            Ok(false)
        }
    }
}

// 删除type_info中的数据通过id
#[allow(dead_code)]
pub fn delete_type_info_by_id(conn: &Connection, id: u64) -> Result<bool> {
    match conn.execute(
        "
        DELETE FROM type_info WHERE id = ?;
    ",
        params![id],
    ) {
        Ok(_) => {
            println!("删除成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("删除失败!{}", e);
            Ok(false)
        }
    }
}

// 查询type_info中的数据通过id
#[allow(dead_code)]
pub fn select_type_info_by_id(conn: &Connection, id: u64) -> Result<Type> {
    let sql = "
        SELECT id, type_name FROM type_info WHERE id = ?
    ";

    let mut stmt = conn.prepare(sql)?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let type_name: String = row.get(1)?;
        Ok(Type { id, type_name })
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}

// 查询 type_info 中的所有数据
#[allow(dead_code)]
pub fn select_type_info(conn: &Connection) -> Result<Vec<Type>> {
    let sql = "
        SELECT id, type_name FROM type_info
    ";

    // 准备查询语句
    let mut stmt = conn.prepare(sql)?;

    // 执行查询并映射所有行到 Type 结构体
    let types = stmt.query_map(params![], |row| {
        Ok(Type {
            id: row.get(0)?,
            type_name: row.get(1)?,
        })
    })?;

    // 将结果转换为 Vec<Type>
    let result: Vec<Type> = types.collect::<Result<Vec<Type>, Error>>()?;

    Ok(result)
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Serialize)]
pub struct WebInfo {
    id: u64,
    name: String,
    address: String,
    type_id: u64,
    is_frequent: bool,
    is_pinned: bool,
    create_time: String,
    type_name: String,
}

impl WebInfo {
    // 公共 getter 方法
    pub fn is_frequent(&self) -> bool {
        self.is_frequent
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

// =========================================web_info========================
// 创建web_info表
#[allow(dead_code)]
pub fn create_web_info_table(conn: &Connection) -> Result<bool> {
    match conn.execute(
        "
                CREATE TABLE IF NOT EXISTS web_info (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    address TEXT NOT NULL,
                    type_id INTEGER,
                    is_frequent BOOLEAN NOT NULL DEFAULT 0,
                    is_pinned BOOLEAN NOT NULL DEFAULT 0,
                    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
        ",
        params![],
    ) {
        Ok(_) => {
            println!("web_info表新增成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("web_info表新增失败!{}", e);
            Ok(false)
        }
    }
}

// 新增数据到web_info
#[allow(dead_code)]
pub fn install_web_info(conn: &Connection, name: &str, address: &str, type_id: u64) -> usize {
    match conn.execute(
        "
        INSERT INTO web_info (name, address, type_id)
        VALUES (?, ?, ?);
    ",
        params![name, address, type_id],
    ) {
        Ok(_) => {
            println!("新增成功!");
            let mut stmt = conn.prepare(" SELECT last_insert_rowid();").unwrap();

            let mut rows = stmt.query(params![]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let id: usize = row.get(0).unwrap();
                id
            } else {
                0
            }
        }
        Err(e) => {
            eprintln!("新增失败!{}", e);
            0
        }
    }
}

// 修改web_info中的数据通过id
#[allow(dead_code)]
pub fn update_web_info_by_id(
    conn: &Connection,
    id: u64,
    name: &str,
    address: &str,
    type_id: u64,
) -> Result<bool> {
    match conn.execute(
        "
        UPDATE web_info
            SET 
        name = ?,
        address = ?,
        type_id = ?
        WHERE id = ?;

    ",
        params![name, address, type_id, id],
    ) {
        Ok(_) => {
            println!("更新成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("更新失败!{}", e);
            Ok(false)
        }
    }
}

// 删除web_info中的数据通过id
#[allow(dead_code)]
pub fn delete_web_info_by_id(conn: &Connection, id: u64) -> Result<bool> {
    match conn.execute(
        "
        DELETE FROM web_info WHERE id = ?;
    ",
        params![id],
    ) {
        Ok(_) => {
            println!("删除成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("删除失败!{}", e);
            Ok(false)
        }
    }
}

// 查询web_info中的数据通过id
#[allow(dead_code)]
pub fn select_web_info_by_id(conn: &Connection, id: u64) -> Result<WebInfo> {
    let sql = "
            SELECT
	            wi.id,
	            wi.name,
	            wi.address,
	            wi.type_id,
	            wi.is_frequent,
	            wi.is_pinned,
	            wi.create_time,
	            IFNULL(ti.type_name, '') AS type_name
                FROM
                	web_info AS wi
                LEFT JOIN type_info AS ti ON ti.id = wi.type_id
                WHERE
                	wi.id = ?
        ";

    let mut stmt = conn.prepare(sql)?;

    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let id: u64 = row.get(0)?;
        let name: String = row.get(1)?;
        let address: String = row.get(2)?;
        let type_id: u64 = row.get(3)?;
        let is_frequent: bool = row.get(4)?;
        let is_pinned: bool = row.get(5)?;
        let create_time: String = row.get(6)?;
        let type_name: String = row.get(7)?;
        Ok(WebInfo {
            id,
            name,
            address,
            type_id,
            is_frequent,
            is_pinned,
            create_time,
            type_name,
        })
    } else {
        Err(Error::QueryReturnedNoRows)
    }
}

// 查询 type_info 中的所有数据
#[allow(dead_code)]
pub fn select_web_info(
    conn: &Connection,
    name: &str,
    address: &str,
    type_name: &str,
) -> Result<Vec<WebInfo>> {
    // 初始化 SQL 查询语句
    let mut sql = "
        SELECT
            wi.id,
            wi.name,
            wi.address,
            wi.type_id,
            wi.is_frequent,
            wi.is_pinned,
            wi.create_time,
            IFNULL(ti.type_name, '') AS type_name
        FROM
            web_info AS wi
        LEFT JOIN type_info AS ti ON ti.id = wi.type_id
        WHERE 1=1"
        .to_string();

    // 用于存储查询参数的向量
    let mut params_vec: Vec<&dyn rusqlite::ToSql> = Vec::new();

    // 创建格式化参数
    let name_param = format!("%{}%", name);
    let address_param = format!("%{}%", address);
    let type_name_param = format!("%{}%", type_name);

    // 动态添加查询条件
    if !name.is_empty() {
        sql.push_str(" AND wi.name LIKE ?");
        params_vec.push(&name_param);
    }

    if !address.is_empty() {
        sql.push_str(" AND wi.address LIKE ?");
        params_vec.push(&address_param);
    }

    if !type_name.is_empty() {
        sql.push_str(" AND ti.type_name LIKE ?");
        params_vec.push(&type_name_param);
    }

    sql.push_str(" ORDER BY wi.is_pinned DESC");

    // 准备查询语句
    let mut stmt = conn.prepare(&sql)?;

    // 执行查询并映射所有行到 WebInfo 结构体
    let web_infos = stmt.query_map(params_vec.as_slice(), |row| {
        Ok(WebInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
            type_id: row.get(3)?,
            is_frequent: row.get(4)?,
            is_pinned: row.get(5)?,
            create_time: row.get(6)?,
            type_name: row.get(7)?,
        })
    })?;

    // 将结果转换为 Vec<WebInfo>
    let result: Vec<WebInfo> = web_infos.collect::<Result<Vec<WebInfo>, Error>>().expect("出现未知错误");
    
    // 输出 SQL 语句用于调试

    Ok(result)
}

#[allow(dead_code)]
pub fn get_web_info_ofent_list(
    conn: &Connection,
    name: &str,
    address: &str,
    type_name: &str,
) -> Result<Vec<WebInfo>> {
    // 初始化 SQL 查询语句
    let mut sql = "
        SELECT
	        wi.id,
	        wi.name,
	        wi.address,
	        wi.type_id,
	        wi.is_frequent,
	        wi.is_pinned,
	        wi.create_time,
	        IFNULL(ti.type_name, '') AS type_name
        FROM
        	web_info AS wi
        	LEFT JOIN type_info AS ti ON ti.id = wi.type_id 
        WHERE
        	1 =1 AND wi.is_frequent = 1"
        .to_string();

    // 用于存储查询参数的向量
    let mut params_vec: Vec<&dyn rusqlite::ToSql> = Vec::new();

    let name_param = format!("%{}%", name);
    format!("%{}%", name);
    let address_param = format!("%{}%", address);
    format!("%{}%", address);
    let type_name_param = format!("%{}%", type_name);
    format!("%{}%", type_name);
    // 动态添加查询条件
    if !name.is_empty() {
        sql.push_str(" AND wi.name like ?");
        params_vec.push(&name_param);
    }

    if !address.is_empty() {
        sql.push_str(" AND wi.address like ?");
        params_vec.push(&address_param);
    }

    if !type_name.is_empty() {
        sql.push_str(" AND ti.type_name like ?");
        params_vec.push(&type_name_param);
    }

    sql.push_str(" ORDER BY wi.is_pinned DESC");

    // 准备查询语句
    let mut stmt = conn.prepare(&sql)?;

    // 执行查询并映射所有行到 WebInfo 结构体
    let web_infos = stmt.query_map(params_vec.as_slice(), |row| {
        Ok(WebInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
            type_id: row.get(3)?,
            is_frequent: row.get(4)?,
            is_pinned: row.get(5)?,
            create_time: row.get(6)?,
            type_name: row.get(7)?,
        })
    })?;

    // 将结果转换为 Vec<WebInfo>
    let result: Vec<WebInfo> = web_infos.collect::<Result<Vec<WebInfo>, Error>>()?;

    Ok(result)
}

// 修改常用
#[allow(dead_code)]
pub fn update_web_info_is_frequent_by_id(
    conn: &Connection,
    id: u64,
    is_frequent: bool,
) -> Result<bool> {
    match conn.execute(
        "
        UPDATE web_info
            SET 
        is_frequent = ?
        WHERE id = ?;

    ",
        params![is_frequent, id],
    ) {
        Ok(_) => {
            println!("更新成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("更新失败!{}", e);
            Ok(false)
        }
    }
}

// 修改置顶
#[allow(dead_code)]
pub fn update_web_info_is_pinned_by_id(
    conn: &Connection,
    id: u64,
    is_pinned: bool,
) -> Result<bool> {
    match conn.execute(
        "
        UPDATE web_info
            SET 
        is_pinned = ?
        WHERE id = ?;

    ",
        params![is_pinned, id],
    ) {
        Ok(_) => {
            println!("更新成功!");
            Ok(true)
        }
        Err(e) => {
            eprintln!("更新失败!{}", e);
            Ok(false)
        }
    }
}
