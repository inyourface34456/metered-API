use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Standered,
    Admin,
    None,
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
