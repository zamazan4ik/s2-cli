//! Types for Basin configuration that directly map to s2::types.

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const STORAGE_CLASS_PATH: &str = "default_stream_config.storage_class";
pub const RETENTION_POLICY_PATH: &str = "default_stream_config.retention_policy";

#[derive(Parser, Debug, Clone, Serialize)]
pub struct BasinConfig {
    #[clap(flatten)]
    pub default_stream_config: Option<StreamConfig>,
}

#[derive(Parser, Debug, Clone, Serialize)]
pub struct StreamConfig {
    #[arg(short, long)]
    /// Storage class for a stream.
    pub storage_class: Option<StorageClass>,
    #[arg(short, long, help("Example: 1d, 1w, 1y"))]
    /// Retention policy for a stream.
    pub retention_policy: Option<RetentionPolicy>,
}

#[derive(ValueEnum, Debug, Clone, Serialize)]
pub enum StorageClass {
    Unspecified,
    Standard,
    Express,
}

#[derive(Clone, Debug, Serialize)]
pub enum RetentionPolicy {
    #[allow(dead_code)]
    Age(Duration),
}

impl From<&str> for RetentionPolicy {
    fn from(s: &str) -> Self {
        match humantime::parse_duration(s) {
            Ok(d) => RetentionPolicy::Age(d),
            Err(_) => RetentionPolicy::Age(Duration::from_secs(0)),
        }
    }
}

impl From<BasinConfig> for s2::types::BasinConfig {
    fn from(config: BasinConfig) -> Self {
        let default_stream_config = config.default_stream_config.map(|c| c.into());
        s2::types::BasinConfig::builder()
            .default_stream_config(default_stream_config)
            .build()
    }
}

impl From<StreamConfig> for s2::types::StreamConfig {
    fn from(config: StreamConfig) -> Self {
        let storage_class = config
            .storage_class
            .map(s2::types::StorageClass::from)
            .unwrap_or(s2::types::StorageClass::Unspecified);
        let retention_policy = config.retention_policy.map(|r| r.into());
        s2::types::StreamConfig::builder()
            .storage_class(storage_class)
            .retention_policy(retention_policy)
            .build()
    }
}

impl From<StorageClass> for s2::types::StorageClass {
    fn from(class: StorageClass) -> Self {
        match class {
            StorageClass::Unspecified => s2::types::StorageClass::Unspecified,
            StorageClass::Standard => s2::types::StorageClass::Standard,
            StorageClass::Express => s2::types::StorageClass::Express,
        }
    }
}

impl From<s2::types::StorageClass> for StorageClass {
    fn from(class: s2::types::StorageClass) -> Self {
        match class {
            s2::types::StorageClass::Unspecified => StorageClass::Unspecified,
            s2::types::StorageClass::Standard => StorageClass::Standard,
            s2::types::StorageClass::Express => StorageClass::Express,
        }
    }
}

impl From<RetentionPolicy> for s2::types::RetentionPolicy {
    fn from(policy: RetentionPolicy) -> Self {
        match policy {
            RetentionPolicy::Age(d) => s2::types::RetentionPolicy::Age(d),
        }
    }
}

impl From<s2::types::RetentionPolicy> for RetentionPolicy {
    fn from(policy: s2::types::RetentionPolicy) -> Self {
        match policy {
            s2::types::RetentionPolicy::Age(d) => RetentionPolicy::Age(d),
        }
    }
}

impl From<s2::types::BasinConfig> for BasinConfig {
    fn from(config: s2::types::BasinConfig) -> Self {
        let default_stream_config = config.default_stream_config.map(|c| c.into());
        BasinConfig {
            default_stream_config,
        }
    }
}

impl From<s2::types::StreamConfig> for StreamConfig {
    fn from(config: s2::types::StreamConfig) -> Self {
        StreamConfig {
            storage_class: Some(config.storage_class.into()),
            retention_policy: config.retention_policy.map(|r| r.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppendRecord {
    /// Series of name-value pairs for this record.    
    pub headers: Vec<Header>,
    /// Body of the record.    
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
}

impl From<AppendRecord> for s2::types::AppendRecord {
    fn from(record: AppendRecord) -> Self {
        let headers = record.headers.into_iter().map(|h| h.into()).collect();
        s2::types::AppendRecord::builder()
            .headers(headers)
            .body(record.body)
            .build()
    }
}

impl From<Header> for s2::types::Header {
    fn from(header: Header) -> Self {
        s2::types::Header::builder()
            .name(header.name)
            .value(header.value)
            .build()
    }
}
