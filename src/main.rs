#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions, AllowedHeaders};


mod model;
mod pgmanager;


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



fn main() {
    pgmanager::load_db();
    pgmanager::create_table();

    let cors = CorsOptions::default()
    .allowed_headers(AllowedHeaders::all())
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::ignite().attach(cors.to_cors().unwrap()).mount("/", routes![set_pane_data, get_pane_data]).launch();
}
