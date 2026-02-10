use core::{num::NonZeroU64, ops::RangeInclusive, time::Duration};

use lb_blend_service::core::settings::ZkSettings;
use lb_libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub backend: BackendConfig,
    pub zk: ZkSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde_as]
pub struct BackendConfig {
    pub listening_address: Multiaddr,
    #[serde(default = "default_core_peering_degree")]
    pub core_peering_degree: RangeInclusive<u64>,
    #[serde_as(
        as = "lb_utils::bounded_duration::MinimalBoundedDuration<1, lb_utils::bounded_duration::SECOND>"
    )]
    #[serde(default = "default_edge_node_connection_timeout")]
    pub edge_node_connection_timeout: Duration,
    #[serde(default = "default_max_edge_node_incoming_connections")]
    pub max_edge_node_incoming_connections: u64,
    #[serde(default = "default_max_dial_attempts_per_peer")]
    pub max_dial_attempts_per_peer: NonZeroU64,
}

const fn default_core_peering_degree() -> RangeInclusive<u64> {
    3..=5
}

const fn default_edge_node_connection_timeout() -> Duration {
    Duration::from_secs(5)
}

const fn default_max_edge_node_incoming_connections() -> u64 {
    100
}

const fn default_max_dial_attempts_per_peer() -> NonZeroU64 {
    NonZeroU64::new(3).unwrap()
}
