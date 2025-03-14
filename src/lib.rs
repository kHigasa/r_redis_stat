use std::collections::HashMap;

extern crate redis;

pub fn get_info(uri: &str) -> RedisInfo {
    let client = redis::Client::open(uri).unwrap();
    let pool = r2d2::Pool::builder().build(client).unwrap();
    let mut con = pool.get().unwrap();

    let res: String = redis::cmd("INFO").query(&mut con).unwrap();
    let res_row = res.split("\n");
    let mut info: HashMap<&str, &str> = HashMap::new();
    for i in res_row {
        if i.contains("#") {
            continue;
        } else {
            let j = i.split_once(":");
            if j == None {
                continue;
            } else {
                let (key, val) = j.unwrap();
                info.insert(key, val.trim());
            }
        }
    }
    RedisInfo::new(info)
}

#[derive(Debug)]
pub struct RedisInfo {
    pub used_cpu_sys: String,
    pub used_cpu_user: String,
    pub connected_clients: String,
    pub blocked_clients: String,
    pub used_memory_human: String,
    pub used_memory_rss_human: String,
    pub keys: String,
    pub total_commands_processed: String,
    pub expired_keys: String,
    pub evicted_keys: String,
    pub keyspace_hits: String,
    pub keyspace_misses: String,
}

impl RedisInfo {
    fn new(info: HashMap<&str, &str>) -> Self {
        Self {
            used_cpu_sys: String::from(info.get("used_cpu_sys").copied().unwrap()), 
            used_cpu_user: String::from(info.get("used_cpu_user").copied().unwrap()),
            connected_clients: String::from(info.get("connected_clients").copied().unwrap()), 
            blocked_clients: String::from(info.get("blocked_clients").copied().unwrap()),
            used_memory_human: String::from(info.get("used_memory_human").copied().unwrap()),
            used_memory_rss_human: String::from(info.get("used_memory_rss_human").copied().unwrap()),
            keys: String::from(info.get("db0").copied().unwrap().split_once(",").unwrap().0.split_once("=").unwrap().1),
            total_commands_processed: String::from(info.get("total_commands_processed").copied().unwrap()),
            expired_keys: String::from(info.get("expired_keys").copied().unwrap()),
            evicted_keys: String::from(info.get("evicted_keys").copied().unwrap()),
            keyspace_hits: String::from(info.get("keyspace_hits").copied().unwrap()),
            keyspace_misses: String::from(info.get("keyspace_misses").copied().unwrap()),
        }
    }
}
