#[derive(Hash, Eq, PartialEq)]
pub struct Endpoint {
    pub name: String,
    pub req_before_cooldown: u16,
    pub cooldown_time: u16,
}

impl Endpoint {
    pub const fn new(name: String, req_before_cooldown: u16, cooldown_time: u16) -> Self {
        Self {
            name,
            req_before_cooldown,
            cooldown_time,
        }
    }
}
