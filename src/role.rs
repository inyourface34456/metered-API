use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Standered,
    Admin,
    None,
}
