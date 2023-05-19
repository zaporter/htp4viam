use std::{
    collections::HashMap,
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

use elasticsearch::Elasticsearch;
use serde::Serialize;

use crate::{config::tests::TestSpecification, htp_test::TestID};
// #[derive(Serialize)]
// struct LokiStream {
//     stream: HashMap<String, String>,
//     values: Vec<[String; 2]>,
// }

// #[derive(Serialize)]
// struct LokiRequest {
//     streams: Vec<LokiStream>,
// }

// struct LokiLogger {
//     url: String,
//     initial_labels: Option<HashMap<String, String>>,
//     client: reqwest::Client,
// }

// impl LokiLogger {
//     fn time_offset_since(start: SystemTime) -> anyhow::Result<String> {
//         let since_start = start.duration_since(UNIX_EPOCH)?;
//         let time_ns = since_start.as_nanos().to_string();
//         Ok(time_ns)
//     }
//     fn make_request(
//         message: String,
//         labels: HashMap<String, String>,
//     ) -> anyhow::Result<LokiRequest> {
//         let start = SystemTime::now();
//         let time_ns = Self::time_offset_since(start)?;
//         let loki_request = LokiRequest {
//             streams: vec![LokiStream {
//                 stream: labels,
//                 values: vec![[time_ns, message], ["chicken".into(), "butt".into()]],
//             }],
//         };
//         Ok(loki_request)
//     }
//     fn log_to_loki(&self, message: String, labels: HashMap<String, String>) -> anyhow::Result<()> {
//         let client = self.client.clone();
//         let url = self.url.clone();

//         let loki_request = Self::make_request(message, labels)?;
//         tokio::spawn(async move {
//             if let Err(e) = client.post(url).json(&loki_request).send().await {
//                 eprintln!("{:?}", e);
//             };
//         });
//         Ok(())
//     }
// }
//
#[derive(Clone, Debug, Serialize)]
pub struct TestRegistrationEntry {
    t_id: String,
    test_name: String,
    creation_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TestCompletionEntry {
    t_id: String,
    test_name: String,
    creation_time: chrono::DateTime<chrono::Utc>,
    execution_start_time: chrono::DateTime<chrono::Utc>,
    termination_time: chrono::DateTime<chrono::Utc>,
    passed: bool,
    stage_success: TestStageSucessEntry,
    test_config: TestSpecification,
    dependencies: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TestStageSucessEntry {
    validation: bool,
    dependency_building: bool,
    resource_aquisition: bool,
    execution: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct UtilizationEntry {
    id: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    cpu_load: f32,
    ram_usage_megabytes: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct LogsEntry {
    id: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    log_num: u64,
    log_msg: String,
    is_stderr: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct StatsEntry<T: Clone + Debug + Serialize> {
    id: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    data: T,
}

#[derive(Clone, Debug, Serialize)]
pub struct DependenciesEntry {
    d_id: String,
    name: String,
    built_for: String,
    build_start: chrono::DateTime<chrono::Utc>,
    build_end: chrono::DateTime<chrono::Utc>,
    fs_root: String,
}

// This was written so that it might
// be easy to one day support multiple databases
#[derive(Debug, Clone)]
pub struct DbWrapper {
    ty: WrapperType,
    id: String,
    client: Elasticsearch,
    log_line_number: u64,
}
#[derive(Debug, Clone)]
pub enum WrapperType {
    Test,
    Dependency,
}
impl DbWrapper {
    pub fn new_elasticsearch(ty: WrapperType, id: String, client: Elasticsearch) -> Self {
        Self {
            ty,
            id,
            client,
            log_line_number: 0,
        }
    }
    pub async fn register_test<S: Into<String>>(&self, test_name: S) {
        let entry = TestRegistrationEntry {
            t_id: self.id.clone(),
            test_name: test_name.into(),
            creation_time: chrono::offset::Utc::now(),
        };
        let response = self
            .client
            .index(elasticsearch::IndexParts::Index("test_registration"))
            .body(entry)
            .send()
            .await;

        if response.is_err() {
            panic!("TODO(ZACK) LOG THIS TO META DB")
        }
    }
    pub async fn log<S: Into<String>>(&mut self, is_stderr: bool, msg: S) {
        let entry = LogsEntry {
            id: self.id.clone(),
            timestamp: chrono::offset::Utc::now(),
            log_num: self.log_line_number,
            log_msg: msg.into(),
            is_stderr,
        };
        let response = self
            .client
            .index(elasticsearch::IndexParts::Index("logs"))
            .body(entry)
            .send()
            .await;

        if response.is_err() {
            panic!("TODO(ZACK) LOG THIS TO META DB")
        }
        self.log_line_number += 1;
    }
}
