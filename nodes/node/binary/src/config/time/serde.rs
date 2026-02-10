use core::{net::Ipv4Addr, time::Duration};

use lb_time_service::backends::{NtpTimeBackendSettings, ntp::async_client::NTPClientSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_ntp_settings")]
    pub backend: NtpTimeBackendSettings,
}

fn default_ntp_settings() -> NtpTimeBackendSettings {
    NtpTimeBackendSettings {
        ntp_client_settings: NTPClientSettings {
            timeout: Duration::from_secs(5),
            listening_interface: Ipv4Addr::UNSPECIFIED.into(),
        },
        ntp_server: "pool.ntp.org:123".to_owned(),
        update_interval: Duration::from_secs(15),
    }
}
