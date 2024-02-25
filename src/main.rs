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
async fn main() {
  /*
  TODO CREATE AN ERROR STRUCT TO HANDLE ALL ERROR
  TODO FIND A WAY TO HAVE CONFIG BE GLOBAL
  TODO CONSIDER Box<str> over String 
  TODO CREATE AN ABSTRATION OF THE SCHEDULER -> HIGH PRIORITY 
  Itertools
  if it's smaller than or equal to usize, copy always.
  Clippy
  */
  env_logger::init();



  let every_1_minute = every(1).minutes().in_timezone(&Utc).perform(|| async {
    let configs = Arc::new(Config::new().await);
    let mut net = Network::default();
    net.get_arp().await;
    net.get_interfaces().await;
    net.get_tcp_info().await;
    net.get_udp_info().await;
    let to_inject_data = ElasticSearchData::from_network(net).await;
    // TODO CAHNGE TO SINGLE PASS 
    inject_data(&to_inject_data, &configs.output.elasticsearch).await;
  });


  let every_10_sec = every(20).seconds().in_timezone(&Utc).perform(|| async {
    let system = Arc::new(Mutex::new(System::default()));
    let (configs,_agent_config) = (Arc::new(Config::new().await),Arc::new(get_agent_config().await));
    let mut threads= Vec::new();

    // GET HOSTINFO BEFORE THREADS
    system.lock().await.get_host_info().await;
    

    let cpu_data = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = cpu_data.lock().await;
        data.get_cpu_stats(5).await;
        data.get_cpu_load_average().await;
      })
    );


    let disk_data = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = disk_data.lock().await;
        data.get_diskio_stats().await;
      })
    );

    let data_memory = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_memory.lock().await;
        data.get_memory_stats().await;
      })
    );

    let data_cgroup = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_cgroup.lock().await;
        data.get_cgroups().await;
      })
    );

    let data_uptime = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_uptime.lock().await;
        data.get_uptime().await;
      })
    );

    let data_filesystems = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_filesystems.lock().await;
        data.get_filesystem().await;
      })
    );

    let data_devices = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_devices.lock().await;
        data.get_devices().await;
      })
    );


    let data_partitons = system.clone();
    threads.push(
      tokio::spawn(async move {
        let mut data = data_partitons.lock().await;
        data.get_partitions().await;
      })
    );


    for thread in threads {
      thread.await.unwrap()
    }

    if let Ok(inner) = Arc::try_unwrap(system) {
      let system = inner.into_inner();
      let to_inject_data = ElasticSearchData::from_system(system).await;
      inject_data(&to_inject_data, &configs.output.elasticsearch).await;
    } else {
      println!("Failed to unwrap system struct");
    }
    
  });

  spawn(every_10_sec);
  spawn(every_1_minute);

  let every_day = every(1).day().in_timezone(&Utc).perform(|| async {
    println!("Scheduled successfully");
  });

  every_day.await;

}
