use oracle::{Connection, ResultSet, Row, SqlValue};

pub trait Exec {
    /// 获得链接
    fn get_connect(&self, username: String, password: String, database: String) -> Connection;
    /// 遍历所有的表，并执行func函数
    fn exec_by_all_tables<F>(&self, rows: ResultSet<Row>, func: F) -> oracle::Result<()> where F: Fn(&SqlValue);
    /// 获得所有的表
    fn get_all_tables<'a>(&self, conn: &'a Connection) -> oracle::Result<ResultSet<'a, Row>>;
    /// 删除数据表
    fn delete_database(&self, conn: &Connection) -> oracle::Result<()>;
    /// 清空数据表
    fn clear_data(&self, conn: &Connection) -> oracle::Result<()>;
    /// 获取指定表结构
    fn desc_table(&self, conn: &Connection, table_name: &str) -> oracle::Result<()>;
    /// 执行sql文件
    fn exec_modify(&self, modify: Vec<String>, conn: &Connection) -> oracle::Result<()>;
}