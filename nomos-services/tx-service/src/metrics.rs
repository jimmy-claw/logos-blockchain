pub fn mempool_transactions_added() {
    nomos_tracing::metric_counter_u64!(mempool_transactions_added_total, 1);
}

pub fn mempool_transactions_removed(removed_count: usize) {
    nomos_tracing::metric_counter_u64!(mempool_transactions_removed_total, removed_count as u64);
}

pub fn mempool_transactions_pending(pending_count: usize) {
    nomos_tracing::metric_gauge_u64!(mempool_transactions_pending, pending_count as u64);
}
