use oracle::{Connection, ResultSet, Row, SqlValue};

pub trait Exec {
    fn get_connect(&self, username: String, password: String, database: String) -> Connection;
    fn exec_by_all_tables<F>(&self, rows: ResultSet<Row>, func: F) -> oracle::Result<()> where F: Fn(&SqlValue);
    fn get_all_tables<'a>(&self, conn: &'a Connection) -> oracle::Result<ResultSet<'a, Row>>;
    fn delete_database(&self, conn: &Connection) -> oracle::Result<()>;
    fn clear_data(&self, conn: &Connection) -> oracle::Result<()>;
    fn desc_table(&self, conn: &Connection, table_name: &str) -> oracle::Result<()>;
    fn exec_modify(&self, modify: Vec<String>, conn: &Connection) -> oracle::Result<()>;
}