use std::thread;
use std::time;

use chrono::{DateTime, Local};

use r_redis_stat::RedisInfo;

const URI: &str = "redis://127.0.0.1/";

fn main() {
    loop {
        let res: RedisInfo = r_redis_stat::get_info(URI);
        let now: DateTime<Local> = Local::now();
        println!("{:?}\n{:#?}", now, res);
        thread::sleep(time::Duration::from_secs(5));
    }
}
