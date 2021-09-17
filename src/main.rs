extern crate oracle;

mod params;

use params::Opts;
use sql_drop_data::oracle_conn::OracleConn;
use sql_drop_data::oracle_conn::interface::Exec;

fn main() -> oracle::Result<()> {
    let Opts { username, password, database, modify, type_app, table_name } = Opts::parse_app();
    let oracle = OracleConn {};
    let conn = oracle.get_connect(username, password, database);
    match type_app.as_str() {
        "delete" => oracle.delete_database(&conn).unwrap(),
        "clear" => oracle.clear_data(&conn).unwrap(),
        "desc" => oracle.desc_table(&conn, &table_name).unwrap(),
        _ => {}
    }
    if modify.len() > 0 {
        oracle.exec_modify(modify, &conn).unwrap();
    }
    conn.close()
}


