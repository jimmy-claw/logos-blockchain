pub fn da_behaviour_event_received(event: &'static str) {
    nomos_tracing::increase_counter_u64!(da_behaviour_events_received_total, 1, event = event);
}

pub fn da_behaviour_share_size_bytes(event: &'static str, share_size: usize) {
    nomos_tracing::metric_histogram_u64!(
        da_behaviour_share_size_bytes,
        u64::try_from(share_size).unwrap_or(u64::MAX),
        event = event
    );
}
