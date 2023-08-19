#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub log_level: String,
    // pub kafka: Kafka,
    // pub redis: Redis,
    // pub compress: Option<Compress>,
}

impl Config {
    pub fn init<P: AsRef<std::path::Path>>(path: P) -> Config {
        let config_text = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config_text).unwrap()
    }
}

// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct Redis {
//     pub addrs: String,
// }

// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct Kafka {
//     pub addrs: Vec<String>,
//     pub producer: Producer,
//     pub consumer: Consumer,
// }

// #[allow(dead_code)]
// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct Producer {
//     pub linger: Option<u64>,
//     pub max_batch_size: usize,
// }

// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct Consumer {
//     pub min_batch_size: i32,
//     pub max_batch_size: i32,
//     pub max_wait_ms: i32,
// }

// #[derive(serde::Deserialize, Debug, Clone)]
// pub struct Compress {
//     pub dict: String,
//     pub level: i32,
// }
