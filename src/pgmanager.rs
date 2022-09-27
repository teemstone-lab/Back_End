#[warn(unused_imports)]
extern crate ini;

use rocket_contrib::json::Json;
use postgres::{Client, NoTls};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;
use ini::Ini;

pub mod dbmodel;

lazy_static! {
    static ref CACHE_DATA: Mutex<HashMap<i32, String>>= {
    let mut m = HashMap::new();
    m.insert(0, "".to_string());
    Mutex::new(m)
  };
}


static mut DBINFO:dbmodel::DBModel = dbmodel::DBModel{
  host: String::new(),
  port: String::new(),
  user: String::new(),
  password: String::new(),
  dbname: String::new(),
};

const INIT_HOST: &str = "127.0.0.1"; 
const INIT_PORT: &str = "5432";  
const INIT_USER: &str = "ontune";  
const INIT_PASSWORD: &str = "ontune";  
const INIT_DBNAME: &str = "webTest";    

pub fn load_db() {
  if Path::new("setting.ini").exists() {   
    let get_conf = Ini::load_from_file("setting.ini").unwrap();
    let section = get_conf.section(Some("db")).unwrap();
    unsafe{
      DBINFO.host = section.get("host").unwrap().to_string();
      DBINFO.port = section.get("port").unwrap().to_string();
      DBINFO.user = section.get("user").unwrap().to_string();
      DBINFO.password = section.get("password").unwrap().to_string();
      DBINFO.dbname = section.get("dbname").unwrap().to_string();
    }
  } else {
    let mut conf = Ini::new();

    conf.with_section(None::<String>)
    .set("encoding", "utf-8");
    conf.with_section(Some("db".to_owned())).set("host", INIT_HOST).set("port", INIT_PORT).set("user", INIT_USER)
    .set("password", INIT_PASSWORD).set("dbname", INIT_DBNAME);
    conf.write_to_file("setting.ini").unwrap();

    unsafe{
      DBINFO.host = INIT_HOST.to_string();
      DBINFO.port = INIT_PORT.to_string();
      DBINFO.user = INIT_USER.to_string();
      DBINFO.password = INIT_PASSWORD.to_string();
      DBINFO.dbname = INIT_DBNAME.to_string();
    }
  }
  CACHE_DATA.lock().unwrap().clear();
}

pub fn get_dbinfo() -> &'static dbmodel::DBModel{
  unsafe{
    &DBINFO
  }
}

pub fn create_table(){
  let str_dbconn: String;
  unsafe {
    str_dbconn = format!("host={} port={}  user={} password={} dbname={}", DBINFO.host, DBINFO.port, DBINFO.user, DBINFO.password, DBINFO.dbname);
  }
  let mut client = Client::connect(&str_dbconn, NoTls).unwrap();

  client.batch_execute("
        CREATE TABLE IF NOT EXISTS tbpane (
          _number          Integer PRIMARY KEY,
          _data            text
          )
    ").unwrap();
  client.close().unwrap();

}

pub fn set_pane_insert(pane_data: Json<dbmodel::SetPaneJsonData>) -> String{
  let str_dbconn: String;
  unsafe {
    str_dbconn = format!("host={} port={}  user={} password={} dbname={}", DBINFO.host, DBINFO.port, DBINFO.user, DBINFO.password, DBINFO.dbname);
  }
  let mut client = Client::connect(&str_dbconn, NoTls).unwrap();

  println!("&pane_data.number == {}", &pane_data.number);
  println!("&pane_data.data == {}", &pane_data.data);
  let _a  = client.execute(
       "INSERT INTO tbpane (_number, _data) VALUES ($1, $2)",
       &[&pane_data.number, &pane_data.data],
  );
  client.close().unwrap();
  format!("{:?}", pane_data)
}

pub fn set_pane_update(pane_data: Json<dbmodel::SetPaneJsonData>) -> String{
  //cache data update
  if CACHE_DATA.lock().unwrap().len() > 0 {
    if CACHE_DATA.lock().unwrap().contains_key(&pane_data.number){
      CACHE_DATA.lock().unwrap().remove(&pane_data.number);
      CACHE_DATA.lock().unwrap().insert(pane_data.number, pane_data.data.clone());
    }
  }
  let str_dbconn: String;
  unsafe {
    str_dbconn = format!("host={} port={}  user={} password={} dbname={}", DBINFO.host, DBINFO.port, DBINFO.user, DBINFO.password, DBINFO.dbname);
  }
  let mut client = Client::connect(&str_dbconn, NoTls).unwrap();

  println!("&pane_data.number == {}", &pane_data.number);
  println!("&pane_data.data == {}", &pane_data.data);
  let _a  = client.execute(
        "UPDATE tbpane SET _data = $2 WHERE _number = $1",
        &[&pane_data.number, &pane_data.data],
  );
  client.close().unwrap();
  format!("{:?}", pane_data)
}

pub fn get_pane_count() -> String {
  let mut cnt_data: String = "".to_string();
  let str_dbconn: String;
  unsafe {
      str_dbconn = format!("host={} port={}  user={} password={} dbname={}", DBINFO.host, DBINFO.port, DBINFO.user, DBINFO.password, DBINFO.dbname);
  }
  let mut client = Client::connect(&str_dbconn, NoTls).unwrap();
  //println!("num == {}", num.to_string());
  for row in client.query("SELECT count(*) as cnt FROM tbpane", &[]).unwrap(){
    let data: i64 = row.get("cnt");
    cnt_data = data.to_string();
  }
  client.close().unwrap();
  cnt_data
}

pub fn get_pane(num: i32) -> String{
  //cache Check
  if CACHE_DATA.lock().unwrap().len() > 0 {
    if CACHE_DATA.lock().unwrap().contains_key(&num){
      let str_cachedata = CACHE_DATA.lock().unwrap().get(&num).unwrap().clone();
      if !str_cachedata.is_empty(){
        println!("cachedata == {}", str_cachedata);
        return str_cachedata;  
      }
    }
  }
  let mut str_returndata: String = "".to_string();
  let str_dbconn: String;
  unsafe {
      str_dbconn = format!("host={} port={}  user={} password={} dbname={}", DBINFO.host, DBINFO.port, DBINFO.user, DBINFO.password, DBINFO.dbname);
  }
  let mut client = Client::connect(&str_dbconn, NoTls).unwrap();
  //println!("num == {}", num.to_string());
  for row in client.query("SELECT _data FROM tbpane where _number=$1", &[&num]).unwrap(){
    let str_data: String = row.get("_data");
    str_returndata = str_data.clone();
    println!("strdata == {}", str_returndata);
    if CACHE_DATA.lock().unwrap().len() > 0 {
      CACHE_DATA.lock().unwrap().clear();
      CACHE_DATA.lock().unwrap().insert(num, str_data);
    } else {
      CACHE_DATA.lock().unwrap().insert(num, str_data);
    }
  } 
  
  client.close().unwrap();
  str_returndata
}