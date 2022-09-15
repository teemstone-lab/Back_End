use serde::Deserialize;
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct SetPaneJsonData {
    pub number: i32,
    pub data: String
}
pub struct DBModel {
  pub host: &'static str,
  pub port: &'static str,
  pub user: &'static str,
  pub password: &'static str,
  pub dbname: &'static str
}