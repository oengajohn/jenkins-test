use utils::{
  read::read_string,
  crypto::decrypt::decrypt,
  types::AgentConfigs
};

pub async fn get_key() -> String {
  read_string("/etc/oenga/.key",false).await.unwrap()
}

pub async fn get_agent_config_encrypted() -> String {
  read_string("/etc/oenga/.config",false).await.unwrap()
}

pub async fn get_agent_config() -> AgentConfigs {
  let decrypted_config = decrypt(&get_agent_config_encrypted().await, &get_key().await).unwrap();
  let configs: AgentConfigs = serde_json::from_str(&decrypted_config).unwrap();
  configs
}