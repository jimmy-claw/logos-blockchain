pub fn consensus_proposals_created_local() {
    nomos_tracing::metric_counter_u64!(consensus_proposals_created_total, 1, origin = "local");
}

pub fn consensus_proposals_create_failed() {
    nomos_tracing::metric_counter_u64!(
        consensus_proposals_create_failed_total,
        1,
        reason = "propose_block"
    );
}
