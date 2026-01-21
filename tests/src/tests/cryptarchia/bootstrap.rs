use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use futures::stream::{self, StreamExt as _};
use nomos_libp2p::PeerId;
use serial_test::serial;
use tests::{
    adjust_timeout,
    common::sync::{wait_for_validators_mode, wait_for_validators_mode_and_height},
    nodes::validator::{Validator, create_validator_config},
    secret_key_to_peer_id,
    topology::configs::{
        create_general_configs_with_blend_core_subset,
        network::{Libp2pNetworkLayout, NetworkParams},
    },
};

#[tokio::test]
#[serial]
async fn test_ibd_behind_nodes() {
    let n_validators = 1;
    let n_initial_validators = 1;

    let network_params = NetworkParams {
        libp2p_network_layout: Libp2pNetworkLayout::Full,
    };
    let general_configs = create_general_configs_with_blend_core_subset(
        n_validators,
        n_initial_validators,
        &network_params,
    );

    let mut initial_validators = vec![];
    for config in general_configs.iter().take(n_initial_validators) {
        let config = create_validator_config(config.clone());
        initial_validators.push(Validator::spawn(config).await.unwrap());
    }

    tokio::time::sleep(Duration::from_secs(30)).await;
}

fn acceptable_height_margin(
    slot_duration: Duration,
    active_slot_coeff: f64,
    duration: Duration,
) -> u64 {
    let block_time = calculate_block_time(slot_duration, active_slot_coeff);
    let margin = duration.div_duration_f64(block_time).ceil() as u64;
    println!(
        "Acceptable height margin:{margin} for duration {duration:?} with block time {block_time:?}"
    );
    margin
}

fn calculate_block_time(slot_duration: Duration, active_slot_coeff: f64) -> Duration {
    println!("slot_duration:{slot_duration:?}, active_slot_coeff:{active_slot_coeff:?}");
    slot_duration.div_f64(active_slot_coeff)
}
