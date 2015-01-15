#![feature(globs)]
extern crate "sqlite3" as sqlite;

use std::os;
use std::io;
use std::io::fs;
use std::io::fs::{PathExtensions};
use sqlite::{open, database};
use sqlite::types::ResultCode::*;

fn main() {

  let mut args = os::args();
  if args.len() < 2 {
    println!("You can 2 oparations.");
    println!("get <id> OR set <id> <password>");
    return;
  }

  let mut database = init_db();

  if !check_table_exists(&mut database) {
    make_table(&mut database);
  }

  match args[1].as_slice() {
    "set" => {
      if args.len() < 4 {
        return;
      }
      set_pass(&mut database, args[2].as_slice(), args[3].as_slice());
    },
    "get" => {
      if args.len() < 3 {
        return;
      }
      get_pass(&mut database, args[2].as_slice())
    },
    "list" => {
      show_ids(&mut database);
    },
    "update" => {
      println!("update");
    },
    _ => return,
  }
}

fn init_db() -> database::Database {

  let home_dir = os::homedir().unwrap();
  let pwbox_dir = format!("{}{}",home_dir.as_str().unwrap(), "/.pwbox");
  let pwbox_path = Path::new( pwbox_dir.as_slice() );

  if !pwbox_path.exists() {
    fs::mkdir(&pwbox_path, io::USER_RWX);
  }

  let db_path = format!("{}/pwbox.sqlite", pwbox_dir);

  let mut db =  match sqlite::open(db_path.as_slice()){
    Ok(db) => db,
    Err(err) => panic!(":( error"),
  };

  return db;
}

fn get_pass(db:&mut database::Database, id: &str) {
  let mut co = match db.prepare( format!("select pass from pwbox where id = '{}'", id).as_slice(),  &None) {
    Ok(s) => s,
    Err(x) => panic!(":( sqlite error {}", db.get_errmsg()),
  };
  co.step();
  if co.get_column_count() > 0 {
    println!("{}", co.get_text(0).unwrap());
  } else {
    println!("Id is unregistered yet");
  }
  co.step();
}

fn set_pass(db:&mut database::Database, id: &str, pass: &str) {
  match db.exec( format!("insert into pwbox (id, pass) values ('{}', '{}')", id, pass).as_slice() ) {
    Ok(..) => println!(":) Successed password save to my pwbox!"),
    Err(x) => panic!(":( sqlite error: {}", db.get_errmsg()),
  }
}

fn make_table(db:&mut database::Database) {
  match db.exec("create table pwbox ( id text, pass text );") {
    Ok(..) => {},
    Err(x) => panic!(":( sqlite error: {}", db.get_errmsg()),
  }
}

fn show_ids(db:&mut database::Database) {
  let mut co = match db.prepare("SELECT * FROM pwbox;", &None) {
    Ok(s) => s,
    Err(err) => panic!(":( sqlite error {}", db.get_errmsg()),
  };
  while co.step() == SQLITE_ROW {
    {
      print!("id:{} ", co.get_text(0).unwrap());
    }
    {
      println!("pass:{} ", co.get_text(1).unwrap());
    }
  }
}

fn check_table_exists(db:&mut database::Database) -> bool{
  let mut co = match db.prepare( "SELECT count(*) as count FROM sqlite_master WHERE 1=1;".as_slice(),  &None) {
    Ok(s) => s,
    Err(x) => panic!(":( sqlite error {}", db.get_errmsg()),
  };

  let mut result = false;

  co.step();
  if co.get_int(0) > 0 {
    result = true;
  }
  co.step();
  return result;
}
