use core::{num::NonZeroUsize, time::Duration};
use std::{collections::HashSet, path::PathBuf};

use lb_chain_leader_service::LeaderWalletConfig;
use lb_chain_network_service::{IbdConfig, OrphanConfig, SyncConfig};
use lb_chain_service::OfflineGracePeriodConfig;
use lb_libp2p::PeerId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub service: ServiceConfig,
    #[serde(default = "default_network_config")]
    pub network: NetworkConfig,
    pub leader: LeaderConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub recovery_file: PathBuf,
    #[serde(default = "default_service_bootstrap")]
    pub bootstrap: lb_chain_service::BootstrapConfig,
}

const fn default_service_bootstrap() -> lb_chain_service::BootstrapConfig {
    lb_chain_service::BootstrapConfig {
        force_bootstrap: false,
        offline_grace_period: OfflineGracePeriodConfig {
            grace_period: Duration::from_mins(20),
            state_recording_interval: Duration::from_mins(1),
        },
        prolonged_bootstrap_period: Duration::from_hours(24),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bootstrap: lb_chain_network_service::BootstrapConfig<PeerId>,
    pub sync: SyncConfig,
}

fn default_network_config() -> NetworkConfig {
    NetworkConfig {
        bootstrap: lb_chain_network_service::BootstrapConfig {
            ibd: IbdConfig {
                delay_before_new_download: Duration::from_secs(10),
                peers: HashSet::new(),
            },
        },
        sync: SyncConfig {
            orphan: OrphanConfig {
                max_orphan_cache_size: NonZeroUsize::new(5).unwrap(),
            },
        },
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderConfig {
    pub wallet: LeaderWalletConfig,
}
