pub fn da_samples_failed() {
    nomos_tracing::increase_counter_u64!(da_samples_failed_total, 1);
}

pub fn da_samples_verified() {
    nomos_tracing::increase_counter_u64!(da_samples_verified_total, 1);
}

pub fn da_blob_requests() {
    nomos_tracing::increase_counter_u64!(da_blob_requests_total, 1);
}

pub fn da_blob_responses() {
    nomos_tracing::increase_counter_u64!(da_blob_responses_total, 1);
}
