use crate::Endpoint;

#[derive(Hash, Eq, PartialEq)]
pub struct Usage {
    pub num_times_used: u32,
    pub next_use_allowed: u64,
    pub allowed: bool,
    pub endpoint: Endpoint,
}

impl Usage {
    pub fn new(name: String, req_before_cooldown: u16, cooldown_time: u16) -> Self {
        Self {
            num_times_used: 0,
            next_use_allowed: 0,
            allowed: true,
            endpoint: Endpoint::new(name, req_before_cooldown, cooldown_time),
        }
    }
}
