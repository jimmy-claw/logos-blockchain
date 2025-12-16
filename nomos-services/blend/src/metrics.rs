pub fn mix_packets_processed_total() {
    nomos_tracing::metric_counter_u64!(blend_mix_packets_processed_total, 1);
}

pub fn peers_connected(count: usize) {
    nomos_tracing::metric_gauge_u64!(blend_peers_connected, count as u64);
}

pub fn outbound_publish_ok() {
    nomos_tracing::metric_counter_u64!(blend_messages_sent_total, 1, action = "publish");
}

pub fn outbound_publish_err() {
    nomos_tracing::metric_counter_u64!(blend_outbound_messages_failed_total, 1, action = "publish");
}

pub fn outbound_forward_ok() {
    nomos_tracing::metric_counter_u64!(blend_messages_sent_total, 1, action = "forward");
}

pub fn outbound_forward_err() {
    nomos_tracing::metric_counter_u64!(blend_outbound_messages_failed_total, 1, action = "forward");
}

pub fn inbound_message_ok() {
    nomos_tracing::metric_counter_u64!(blend_messages_received_total, 1);
}

pub fn inbound_message_err() {
    nomos_tracing::metric_counter_u64!(blend_inbound_messages_failed_total, 1);
}
