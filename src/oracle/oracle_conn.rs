use oracle::{Connection, ResultSet, Row, SqlValue};

#[path = "../interface.rs"]
pub mod interface;

pub struct OracleConn {}

impl interface::Exec for OracleConn {
    /// 获得链接
    fn get_connect(&self, username: String, password: String, database: String) -> Connection {
        let mut conn = Connection::connect(username, password, database).unwrap();
        conn.set_autocommit(true);
        conn
    }
    /// 遍历所有的表，并执行func函数
    fn exec_by_all_tables<F>(&self, rows: ResultSet<Row>, func: F) -> oracle::Result<()> where F: Fn(&SqlValue) {
        for row_result in rows {
            // print column values
            for (_, val) in row_result.unwrap().sql_values().iter().enumerate() {
                // let x = val.to_string().as_str();
                func(val);
            }
        }
        Ok(())
    }
    /// 获得所有的表
    fn get_all_tables<'a>(&self, conn: &'a Connection) -> oracle::Result<ResultSet<'a, Row>> {
        let sql = "select table_name from user_tables";
        conn.query(sql, &[])
    }
    /// 删除数据表
    fn delete_database(&self, conn: &Connection) -> oracle::Result<()> {
        let rows = self.get_all_tables(conn)?;
        let delete = |val: &SqlValue| {
            conn.execute(format!("drop table {}", val.to_string()).as_str(), &[]).unwrap();
        };
        self.exec_by_all_tables(rows, delete)
    }
    /// 清空数据表
    fn clear_data(&self, conn: &Connection) -> oracle::Result<()> {
        let rows = self.get_all_tables(conn)?;
        let clear = |val: &SqlValue| {
            conn.execute(format!("truncate table {}", val.to_string()).as_str(), &[]).unwrap();
        };
        self.exec_by_all_tables(rows, clear)
    }
    /// 获取指定表结构
    fn desc_table(&self, conn: &Connection, table_name: &str) -> oracle::Result<()> {
        conn.execute(format!("desc {}", table_name).as_str(), &[]).unwrap();
        Ok(())
    }

    /// 执行sql文件
    fn exec_modify(&self, modify: Vec<String>, conn: &Connection) -> oracle::Result<()> {
        let mut sqls = vec![];
        let mut str = String::new();
        for x in modify {
            let str = &mut str;
            let content = std::fs::read_to_string(x).expect("文件不存在");
            for x in content.lines() {
                let x1: Vec<&str> = x.trim_end().split(';').collect();
                if x1.len() > 1 {
                    str.push_str(&x1[0]);
                    sqls.push(str.clone());
                    str.clear();
                    str.push_str(&x1[1]);
                } else {
                    str.push_str(&x1[0]);
                }
            }
        }
        for x in sqls {
            println!("{}", x);
            conn.execute(x.as_str(), &[])?;
        }
        conn.commit()
    }
}
