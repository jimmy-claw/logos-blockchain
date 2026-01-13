pub fn kms_register_requests() {
    nomos_tracing::increase_counter_u64!(kms_register_requests_total, 1);
}

pub fn kms_register_failures() {
    nomos_tracing::increase_counter_u64!(kms_register_failures_total, 1);
}

pub fn kms_register_success() {
    nomos_tracing::increase_counter_u64!(kms_register_success_total, 1);
}

pub fn kms_public_key_requests() {
    nomos_tracing::increase_counter_u64!(kms_public_key_requests_total, 1);
}

pub fn kms_sign_requests_single() {
    nomos_tracing::increase_counter_u64!(kms_sign_requests_total, 1, strategy = "single");
}

pub fn kms_sign_requests_multi() {
    nomos_tracing::increase_counter_u64!(kms_sign_requests_total, 1, strategy = "multi");
}

pub fn kms_sign_failures_single() {
    nomos_tracing::increase_counter_u64!(kms_sign_failures_total, 1, strategy = "single");
}

pub fn kms_sign_failures_multi() {
    nomos_tracing::increase_counter_u64!(kms_sign_failures_total, 1, strategy = "multi");
}

pub fn kms_sign_success_single() {
    nomos_tracing::increase_counter_u64!(kms_sign_success_total, 1, strategy = "single");
}

pub fn kms_sign_success_multi() {
    nomos_tracing::increase_counter_u64!(kms_sign_success_total, 1, strategy = "multi");
}

pub fn kms_sign_single_result<T, E>(result: &Result<T, E>) {
    if result.is_err() {
        kms_sign_failures_single();
    } else {
        kms_sign_success_single();
    }
}

pub fn kms_sign_multi_result<T, E>(result: &Result<T, E>) {
    if result.is_err() {
        kms_sign_failures_multi();
    } else {
        kms_sign_success_multi();
    }
}

pub fn kms_execute_requests() {
    nomos_tracing::increase_counter_u64!(kms_execute_requests_total, 1);
}

pub fn kms_execute_failures() {
    nomos_tracing::increase_counter_u64!(kms_execute_failures_total, 1);
}
