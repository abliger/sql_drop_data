use oracle::{Connection, ResultSet, Row, SqlValue};

/// 获得所有的表
fn get_all_tables(conn: &Connection) ->oracle::Result<ResultSet<Row>> {
    let sql = "select table_name from user_tables";
    let rows = conn.query(sql, &[])?;
    Ok(rows)
}
/// 遍历所有的表，并执行func函数
fn exec_by_all_tables<F>(rows: ResultSet<Row>, func: F) ->oracle::Result<()> where F: Fn(&SqlValue){
    for row_result in rows {
        // print column values
        for (_, val) in row_result.unwrap().sql_values().iter().enumerate() {
            // let x = val.to_string().as_str();
            func(val);
        }
    }
    Ok(())
}
/// 获得链接
pub fn get_connect(username: String, password: String, database: String) -> Connection {
    let mut conn = Connection::connect(username, password, database).unwrap();
    conn.set_autocommit(true);
    conn
}
/// 删除数据表
pub fn delete_database(conn: &Connection) ->oracle::Result<()>{
    let rows = get_all_tables(conn)?;
    let delete=|val:&SqlValue|{
        conn.execute(format!("drop table {}", val.to_string()).as_str(), &[]).unwrap();
    };
    exec_by_all_tables(rows,delete)
}
/// 清空数据表
pub fn clear_data(conn: &Connection) ->oracle::Result<()>{
    let rows = get_all_tables(conn)?;
    let clear=|val:&SqlValue|{
        conn.execute(format!("truncate table {}", val.to_string()).as_str(), &[]).unwrap();
    };
    exec_by_all_tables(rows,clear)
}
/// 获取指定表结构
pub fn desc_table(conn: &Connection,table_name: &str) ->oracle::Result<()>{
    conn.execute(format!("desc {}", table_name).as_str(), &[])?;
    Ok(())
}

/// 执行sql文件
pub fn exec_modify(modify: Vec<String>, conn: &Connection)->oracle::Result<()> {
    for x in modify {
        let content = std::fs::read_to_string(x).expect("文件不存在");
        conn.batch(&content, std::u32::MAX as usize).build().unwrap().execute()?;
    }
    Ok(())
}