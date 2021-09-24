extern crate oracle;
extern crate sql_drop_data;

mod params;

use params::Opts;
use sql_drop_data::oracle_conn::OracleConn;
use sql_drop_data::oracle_conn::interface::Exec;

fn main(){
    let Opts { username, password, database, modify, type_app, table_name } = Opts::parse_app();
    let ora = OracleConn {};
    let conn = ora.get_connect(username, password, database);
    match type_app.as_str() {
        "delete" => ora.delete_database(&conn).unwrap(),
        "clear" => ora.clear_data(&conn).unwrap(),
        "desc" => ora.desc_table(&conn, &table_name).unwrap(),
        _ => {}
    }
    if modify.len() > 0 {
        ora.exec_modify(modify, &conn).unwrap();
    }
}


