extern crate oracle;

mod params;

use params::Opts;
use sql_drop_data::{get_connect, clear_data, exec_modify, delete_database,desc_table};

fn main() -> oracle::Result<()> {
    let Opts { username, password, database, modify, type_app,table_name } = Opts::parse_app();
    println!("{}", table_name);
    let conn = get_connect(username, password, database);
    match type_app.as_str() {
        "delete" => delete_database(&conn).unwrap(),
        "clear" => clear_data(&conn).unwrap(),
        "desc"=> desc_table(&conn,&table_name).unwrap(),
        _ => {}
    }
    if modify.len() > 0 {
        exec_modify(modify, &conn).unwrap();
    }
    conn.close()
}


