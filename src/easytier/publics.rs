use crate::controller::Room;

pub type PublicServers = Vec<String>;

pub fn fetch_public_nodes(_: &Room, mut external_nodes: PublicServers) -> PublicServers {
    external_nodes.extend_from_slice(&[
        "tcp://47.112.97.138:11010",
    ].map(|s| s.into()));

    external_nodes
}
