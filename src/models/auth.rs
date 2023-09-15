use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct get_json_data{
    pub id: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub pincode: Option<String>
}
#[derive(Deserialize, sqlx::FromRow)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
}
