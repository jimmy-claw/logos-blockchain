use std::time::Instant;

pub fn da_verifier_share_requests() {
    nomos_tracing::increase_counter_u64!(da_verifier_share_requests_total, 1);
}

pub fn da_verifier_share_failed() {
    nomos_tracing::increase_counter_u64!(da_verifier_share_failed_total, 1);
}

pub fn da_verifier_observe_share_ok(started_at: Instant) {
    nomos_tracing::metric_histogram_f64!(
        da_verifier_share_handle_duration_seconds,
        started_at.elapsed().as_secs_f64()
    );
}

pub fn da_verifier_tx_requests() {
    nomos_tracing::increase_counter_u64!(da_verifier_tx_requests_total, 1);
}

pub fn da_verifier_tx_failed() {
    nomos_tracing::increase_counter_u64!(da_verifier_tx_failed_total, 1);
}

pub fn da_verifier_observe_tx_ok(started_at: Instant) {
    nomos_tracing::metric_histogram_f64!(
        da_verifier_tx_handle_duration_seconds,
        started_at.elapsed().as_secs_f64()
    );
}

pub fn da_verifier_prune_runs() {
    nomos_tracing::increase_counter_u64!(da_verifier_prune_runs_total, 1);
}

pub fn da_verifier_prune_failed() {
    nomos_tracing::increase_counter_u64!(da_verifier_prune_failed_total, 1);
}

pub fn da_verifier_observe_prune_ok(started_at: Instant) {
    nomos_tracing::metric_histogram_f64!(
        da_verifier_prune_duration_seconds,
        started_at.elapsed().as_secs_f64()
    );
}
