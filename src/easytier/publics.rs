use crate::controller::Room;
use std::{env, fs, path::PathBuf};

pub type PublicServers = Vec<String>;

fn read_nodes_from_config() -> Option<Vec<String>> {
    let config_path = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("terracotta-nodes.txt"))
        .unwrap_or_else(|| PathBuf::from("terracotta-nodes.txt"));

    let content = fs::read_to_string(&config_path).ok()?;

    let nodes: Vec<String> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|s| s.to_string())
        .collect();

    if nodes.is_empty() {
        None
    } else {
        Some(nodes)
    }
}

pub fn fetch_public_nodes(_: &Room, mut external_nodes: PublicServers) -> PublicServers {
    match read_nodes_from_config() {
        Some(nodes) => {
            external_nodes.extend(nodes);
        }
        None => {
            // 默认节点（当配置文件不存在或为空时使用）
            external_nodes.extend_from_slice(&[
                "tcp://47.112.97.138:11010",
            ].map(|s| s.into()));
        }
    }

    external_nodes
}
