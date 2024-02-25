mod setup;
use std::sync::Arc;
use tokio::sync::Mutex;
use config::config::types::Config;
use exporters::elasticsearch::{
  types::ElasticSearchData,
  bulk_insert::inject_data
};
use setup::get_agent_config;
use system::types::System;
use net::types::Network;
use chrono::Utc;
use tokio::spawn;
use tokio_schedule::{
  every, 
  Job
};
use env_logger;

#[tokio::main]