use std::path::PathBuf;

use ::core::num::NonZeroU64;
use lb_key_management_system_service::backend::preload::KeyId;
use lb_libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

use crate::config::blend::serde::{
    core::Config as CoreConfig,
    edge::{BackendConfig as EdgeBackendConfig, Config as EdgeConfig},
};

pub mod core;
pub mod edge;

/// Config object that is part of the global config file.
///
/// This includes all values that are not strictly related to any specific
/// deployment and that users have to specify when starting up the node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// The non-ephemeral signing key (NSK) ID corresponding to the public key
    /// registered in the membership (SDP).
    pub non_ephemeral_signing_key_id: KeyId,
    pub recovery_path_prefix: PathBuf,
    pub core: CoreConfig,
    #[serde(default = "default_edge_config")]
    pub edge: EdgeConfig,
}

const fn default_edge_config() -> EdgeConfig {
    EdgeConfig {
        backend: EdgeBackendConfig {
            max_dial_attempts_per_peer_per_message: NonZeroU64::new(3).unwrap(),
            replication_factor: NonZeroU64::new(3).unwrap(),
        },
    }
}

impl Config {
    pub fn set_listening_address(&mut self, addr: Multiaddr) {
        self.core.backend.listening_address = addr;
    }
}
