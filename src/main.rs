extern crate oracle;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{AppSettings, Clap};
use oracle::Connection;

#[derive(Clap)]
#[clap(version = "1.0", author = "fengsixue <3330181534@qq.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = 'u', long)]
    username: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short = 'p', long)]
    password: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short = 'd', long, default_value = "//118.31.18.193:1531/helowin")]
    database: String,
    #[clap(short = 'm', long)]
    modify: Vec<String>,
}

fn main() {
    let Opts { username, password, database, modify } = Opts::parse();

    let conn = Connection::connect(username, password, database).unwrap();
    conn.autocommit();
    let sql = "select table_name from user_tables";
    let rows = conn.query(sql, &[]).unwrap();

    for row_result in rows {
        // print column values
        for (_, val) in row_result.unwrap().sql_values().iter().enumerate() {
            // let x = val.to_string().as_str();
            conn.execute(format!("truncate table {}", val.to_string()).as_str(), &[]).unwrap();
        }
    }

    for x in modify {
        let input = File::open(&x).expect("文件不存在");
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            if let Ok(string) = line {
                if string.len() > 0 {
                    println!("{}", &string[0..string.len()]);
                    conn.execute(&string[0..string.len() - 1], &[]).unwrap();
                }
            } else {
                panic!("解析失败");
            };
        }
        conn.commit();
    }
    conn.close();
}
