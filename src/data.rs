use serde::Deserialize;
use crate::Role;

#[derive(Deserialize)]
pub struct Data<T: std::marker::Send> {
    pub authentication: u128,
    pub data: T,
}

impl From<Role> for String {
    fn from(data: Role) -> String {
        match data {
            Role::Standered => String::from("Standard"),
            Role::Admin => String::from("Admin"),
            Role::None => String::from("None"),
        }
    }
}