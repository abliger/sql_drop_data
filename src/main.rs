extern crate oracle;

mod params;

use params::Opts;
use sql_drop_data::{get_connect, clear_data, exec_modify, delete_database};

fn main() -> oracle::Result<()> {
    let Opts { username, password, database, modify, type_app } = Opts::parse_app();
    let conn = get_connect(username, password, database);
    match type_app.as_str() {
        "delete" => delete_database(&conn).unwrap(),
        "clear" => clear_data(&conn).unwrap(),
        _ => panic!("type属性请使用 clear .. 等")
    }
    if modify.len() > 0 {
        exec_modify(modify, &conn).unwrap();
    }
    conn.close()
}


