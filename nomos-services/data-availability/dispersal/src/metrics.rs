pub fn da_dispersal_requests() {
    nomos_tracing::metric_counter_u64!(da_dispersal_requests_total, 1);
}

pub fn da_dispersal_payload_bytes(bytes: u64) {
    nomos_tracing::metric_histogram_u64!(da_dispersal_payload_bytes, bytes);
}

pub fn da_dispersal_requests_failed_session_unavailable() {
    nomos_tracing::metric_counter_u64!(
        da_dispersal_requests_failed_total,
        1,
        reason = "session_unavailable"
    );
}

pub fn da_dispersal_requests_failed_process() {
    nomos_tracing::metric_counter_u64!(da_dispersal_requests_failed_total, 1, reason = "process");
}

pub fn da_dispersal_retry_success() {
    nomos_tracing::metric_counter_u64!(da_dispersal_retry_success_total, 1);
}

pub fn da_dispersal_retry_failed() {
    nomos_tracing::metric_counter_u64!(da_dispersal_retry_failed_total, 1);
}
