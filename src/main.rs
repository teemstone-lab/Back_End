#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]

extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions, AllowedHeaders};

mod model;
mod pgmanager;
mod skthread;

#[macro_export]
macro_rules! skmacro{
    () =>{
        println!("No Arg"); 
    };
    (arg => $arg:expr) =>{
        println!("One == {}", $arg);
    };
    ($arg:expr, $arg2:expr) =>{
        println!("One == {}  Two == {}", $arg, $arg2);
    };
}

#[post("/setPane", format = "application/json", data = "<pane_data>")]
fn set_pane_data(pane_data: Json<pgmanager::dbmodel::SetPaneJsonData>) -> String {
    let str_panedata: String;
    let str_dbdata: String = pgmanager::get_pane(pane_data.number);
    
    if str_dbdata.is_empty() {
        str_panedata = pgmanager::set_pane_insert(pane_data);
        format!("Insert Data {:?}", str_panedata)
    } else {
        str_panedata = pgmanager::set_pane_update(pane_data); 
        format!("Update Data {:?}", str_panedata)
    } 
}

#[post("/getPane/<num>")]
fn get_pane_data(num: i32) -> String {
    let str_panedata: String = pgmanager::get_pane(num);
    str_panedata
}

#[get("/getPane/count")]
fn get_pane_count() -> String {
    let pane_count: String = pgmanager::get_pane_count();
    pane_count
}

fn main() {
    pgmanager::load_db();
    pgmanager::create_table();
    
    skmacro!();
    skmacro!(arg => "1234");
    skmacro!("Hoho", 3.14);

    skthread::sk_threadstart(pgmanager::get_dbinfo());

    let cors = CorsOptions::default()
    .allowed_headers(AllowedHeaders::all())
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::ignite().attach(cors.to_cors().unwrap()).mount("/", routes![get_index, set_pane_data, get_pane_data, get_pane_count]).launch();
    
}
