use std::time::{Duration, Instant};

use chain_service::api::ApiError;
use overwatch::DynError;

pub fn consensus_proposals_received_total(origin: &'static str) {
    nomos_tracing::metric_counter_u64!(consensus_proposals_received_total, 1, origin = origin);
}

pub fn consensus_proposals_ignored_total(reason: &'static str, origin: &'static str) {
    nomos_tracing::metric_counter_u64!(
        consensus_proposals_ignored_total,
        1,
        reason = reason,
        origin = origin
    );
}

pub fn consensus_apply_block_failed_total(reason: &'static str) {
    nomos_tracing::metric_counter_u64!(consensus_apply_block_failed_total, 1, reason = reason);
}

pub fn consensus_observe_apply_block_ok(duration: Duration) {
    nomos_tracing::metric_histogram_f64!(consensus_apply_block_seconds, duration.as_secs_f64());
}

pub fn consensus_observe_apply_block_err(err: &ApiError) {
    let reason = match err {
        ApiError::ParentMissing { .. } => "parent_missing",
        _ => "other",
    };
    consensus_apply_block_failed_total(reason);
}

pub fn consensus_observe_proposal_reconstruct_ok(duration: Duration) {
    nomos_tracing::metric_histogram_f64!(
        consensus_proposal_reconstruct_seconds,
        duration.as_secs_f64()
    );
}

pub fn consensus_observe_proposal_reconstruct_err(origin: &'static str, reason: &'static str) {
    nomos_tracing::metric_counter_u64!(
        consensus_proposal_reconstruct_failed_total,
        1,
        reason = reason,
        origin = origin
    );
}

pub fn consensus_observe_proposal_missing_txs(count: usize) {
    nomos_tracing::metric_histogram_u64!(consensus_proposal_missing_txs, count as u64);
}

pub fn orphan_blocks_queue_full_total() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_queue_full_total, 1);
}

pub fn orphan_blocks_enqueued_total() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_enqueued_total, 1);
}

pub fn orphan_blocks_pending(count: usize) {
    nomos_tracing::metric_gauge_u64!(orphan_blocks_pending, count as u64);
}

pub fn orphan_observe_parent_fetch_ok(duration: Duration) {
    nomos_tracing::metric_histogram_f64!(
        orphan_blocks_parent_fetch_seconds,
        duration.as_secs_f64()
    );
}

pub fn orphan_observe_parent_fetch_err() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_parent_fetch_failed_total, 1);
}

pub fn orphan_blocks_removed_total() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_removed_total, 1);
}

pub fn orphan_blocks_received_total() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_received_total, 1);
}

pub fn orphan_blocks_fetch_failed_total() {
    nomos_tracing::metric_counter_u64!(orphan_blocks_fetch_failed_total, 1);
}

pub fn consensus_block_blob_validation_failed_total(mode: &'static str, reason: &'static str) {
    nomos_tracing::metric_counter_u64!(
        consensus_block_blob_validation_failed_total,
        1,
        mode = mode,
        reason = reason
    );
}

pub fn consensus_observe_block_blob_validation_ok(started_at: Instant, mode: &'static str) {
    nomos_tracing::metric_histogram_f64!(
        consensus_block_blob_validation_seconds,
        started_at.elapsed().as_secs_f64(),
        mode = mode
    );
}

pub fn chainsync_observe_download_blocks_ok(duration: Duration, blocks_downloaded: u64) {
    nomos_tracing::metric_counter_u64!(chainsync_requests_total, 1, kind = "blocks", result = "ok");
    nomos_tracing::metric_histogram_f64!(chainsync_download_blocks_seconds, duration.as_secs_f64());
    nomos_tracing::metric_histogram_u64!(chainsync_download_blocks_blocks, blocks_downloaded);
}

pub fn chainsync_observe_download_blocks_err() {
    nomos_tracing::metric_counter_u64!(
        chainsync_requests_total,
        1,
        kind = "blocks",
        result = "err"
    );
}

pub fn chainsync_observe_request_tip_ok(duration: Duration) {
    nomos_tracing::metric_counter_u64!(chainsync_requests_total, 1, kind = "tip", result = "ok");
    nomos_tracing::metric_histogram_f64!(chainsync_request_tip_seconds, duration.as_secs_f64());
}

pub fn chainsync_observe_request_tip_err() {
    nomos_tracing::metric_counter_u64!(chainsync_requests_total, 1, kind = "tip", result = "err");
}

pub fn chainsync_observe_request_tip<T>(
    duration: Duration,
    result: Result<T, DynError>,
) -> Result<T, DynError> {
    if result.is_ok() {
        chainsync_observe_request_tip_ok(duration);
    } else {
        chainsync_observe_request_tip_err();
    }
    result
}
