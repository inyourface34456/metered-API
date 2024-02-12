use crate::{get_unix_epoch, Role, Usage};
use rand::Rng;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Ids {
    pub id_list: Arc<RwLock<HashMap<u128, Role>>>,
    pub usage_list: Arc<RwLock<HashMap<u128, HashMap<String, Usage>>>>,
    pub api_3_data: Arc<RwLock<Vec<i32>>>,
}

impl Ids {
    // You can make A hits until you have to wait B mins
    const API_LIMITS: [(&'static str, u16, u16); 4] = [
        ("/short_wait", 10, 1),
        ("/long_wait", 1, 100),
        ("/add_to_list", 60, 1),
        ("/echo", 60, 1),
    ];

    pub fn new() -> Self {
        Self {
            id_list: Arc::new(RwLock::new(HashMap::new())),
            usage_list: Arc::new(RwLock::new(HashMap::new())),
            api_3_data: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn gen_start_hashmap(role: Role) -> HashMap<String, Usage> {
        let mut map = HashMap::new();

        match role {
            Role::Standered => {
                for i in Self::API_LIMITS {
                    map.insert(String::from(i.0), Usage::new(String::from(i.0), i.1, i.2));
                }
            }
            Role::Admin => {
                for i in Self::API_LIMITS {
                    map.insert(
                        String::from(i.0),
                        Usage::new(String::from(i.0), u16::MAX, 0),
                    );
                }
            }
            Role::None => {}
        }

        map
    }

    pub fn gen_new_id(&self, role: Role) -> u128 {
        let mut rng = rand::thread_rng();
        let mut correct_id: u128 = 0;

        if role == Role::None {
            return 0;
        }

        match self.id_list.write() {
            Ok(mut map) => {
                loop {
                    let id: u128 = rng.gen();
                    if let Entry::Vacant(e) = map.entry(id) {
                        e.insert(role);
                        correct_id = id;
                        break;
                    }
                }

                match self.usage_list.write() {
                    Ok(mut map) => {
                        map.insert(correct_id, Self::gen_start_hashmap(role));
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
        correct_id
    }

    pub fn register_hit(&self, user: u128, endpoint: &str) -> (bool, Role) {
        let mut allowed: bool = true;
        let mut role: Role = Role::None;

        match self.id_list.read() {
            Ok(map) => match map.get(&user) {
                Some(dat) => role = *dat,
                None => {}
            },
            Err(_) => {}
        }

        match self.usage_list.write() {
            Ok(mut map) => match map.get_mut(&user) {
                Some(dat) => match dat.get_mut(endpoint) {
                    Some(dat) => {
                        let current_time = get_unix_epoch();

                        if dat.next_use_allowed > current_time {
                            allowed = false;
                            dat.allowed = false;
                        } else if dat.num_times_used >= dat.endpoint.req_before_cooldown.into() {
                            dat.next_use_allowed = current_time
                                + <u16 as Into<u64>>::into(dat.endpoint.cooldown_time * 60);
                            allowed = false;
                            dat.allowed = false;
                            dat.num_times_used = 0;
                        } else {
                            allowed = true;
                            dat.allowed = true;
                            dat.num_times_used += 1;
                        }
                    }
                    None => allowed = false,
                },
                None => allowed = false,
            },
            Err(_) => {}
        }

        (allowed, role)
    }

    pub fn time_until_next_allowed_hit(&self, user: u128, endpoint: &str) -> Option<u64> {
        match self.usage_list.read() {
            Ok(dat) => match dat.get(&user) {
                Some(dat) => match dat.get(&endpoint.to_string()) {
                    Some(dat) => {
                        let current_time = get_unix_epoch();

                        if dat.next_use_allowed > current_time {
                            Some(dat.next_use_allowed - current_time)
                        } else {
                            Some(0)
                        }
                    }
                    None => None,
                },
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn num_hits_untill_timeout(&self, user: u128, endpoint: &str) -> Option<u32> {
        match self.usage_list.read() {
            Ok(dat) => match dat.get(&user) {
                Some(dat) => match dat.get(&endpoint.to_string()) {
                    Some(dat) => {
                        if dat.allowed {
                            Some(dat.endpoint.req_before_cooldown as u32 - dat.num_times_used)
                        } else {
                            Some(0)
                        }
                    }
                    None => None,
                },
                None => None,
            },
            Err(_) => None,
        }
    }
}
