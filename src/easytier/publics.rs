use crate::controller::Room;
use serde::Deserialize;
use std::{env, fs, path::PathBuf};

pub type PublicServers = Vec<String>;

#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    nodes: Vec<String>,
}

fn read_nodes_from_config() -> Option<Vec<String>> {
    let config_path = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("config.json"))
        .unwrap_or_else(|| PathBuf::from("config.json"));

    let content = fs::read_to_string(&config_path).ok()?;
    let config: Config = serde_json::from_str(&content).ok()?;

    if config.nodes.is_empty() {
        None
    } else {
        Some(config.nodes)
    }
}

pub fn fetch_public_nodes(_: &Room, mut external_nodes: PublicServers) -> PublicServers {
    match read_nodes_from_config() {
        Some(nodes) => {
            external_nodes.extend(nodes);
        }
        None => {
            // 默认节点（当配置文件不存在或解析失败时使用）
            external_nodes.extend_from_slice(&[
                "tcp://47.112.97.138:11010",
            ].map(|s| s.into()));
        }
    }

    external_nodes
}
