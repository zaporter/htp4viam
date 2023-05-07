use std::{
    collections::HashMap,
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

use crate::htp_test::TestID;
#[derive(Serialize)]
struct LokiStream {
    stream: HashMap<String, String>,
    values: Vec<[String; 2]>,
}

#[derive(Serialize)]
struct LokiRequest {
    streams: Vec<LokiStream>,
}

struct LokiLogger {
    url: String,
    initial_labels: Option<HashMap<String, String>>,
    client: reqwest::Client,
}

impl LokiLogger {
    fn time_offset_since(start: SystemTime) -> anyhow::Result<String> {
        let since_start = start.duration_since(UNIX_EPOCH)?;
        let time_ns = since_start.as_nanos().to_string();
        Ok(time_ns)
    }
    fn make_request(
        message: String,
        labels: HashMap<String, String>,
    ) -> anyhow::Result<LokiRequest> {
        let start = SystemTime::now();
        let time_ns = Self::time_offset_since(start)?;
        let loki_request = LokiRequest {
            streams: vec![LokiStream {
                stream: labels,
                values: vec![[time_ns, message], ["chicken".into(), "butt".into()]],
            }],
        };
        Ok(loki_request)
    }
    fn log_to_loki(&self, message: String, labels: HashMap<String, String>) -> anyhow::Result<()> {
        let client = self.client.clone();
        let url = self.url.clone();

        let loki_request = Self::make_request(message, labels)?;
        tokio::spawn(async move {
            if let Err(e) = client.post(url).json(&loki_request).send().await {
                eprintln!("{:?}", e);
            };
        });
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct StatsSink {
    key: TestID,
}
impl StatsSink {
    pub fn new(key: TestID) -> Self {
        Self { key }
    }
    pub fn write<T>(&mut self, stage: &str, val: T)
    where
        T: Serialize + Debug + Clone + std::fmt::Display,
    {
        let logger = LokiLogger {
            initial_labels: None,
            url: "http://localhost:3030/loki/api/v1/push".to_string(),
            client: reqwest::Client::new(),
        };
        println!("STATS: {}, {}, {:?}", self.key, stage, val);
        let mut labels = HashMap::new();
        labels.insert("type".into(), "zack".into());
        logger
            .log_to_loki(format!("{}-{}", stage, val), labels)
            .unwrap();
    }
}
