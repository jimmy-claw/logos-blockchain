use crate::WalletMsg;

pub fn wallet_request(msg: &WalletMsg) {
    let kind = match msg {
        WalletMsg::GetBalance { .. } => "get_balance",
        WalletMsg::FundAndSignTx { .. } => "fund_and_sign_tx",
        WalletMsg::GetLeaderAgedNotes { .. } => "get_leader_aged_notes",
    };

    nomos_tracing::metric_counter_u64!(wallet_requests_total, 1, kind = kind);
}

pub fn wallet_response_send_failed_get_balance() {
    nomos_tracing::metric_counter_u64!(wallet_response_send_failed_total, 1, kind = "get_balance");
}

pub fn wallet_response_send_failed_fund_and_sign_tx() {
    nomos_tracing::metric_counter_u64!(
        wallet_response_send_failed_total,
        1,
        kind = "fund_and_sign_tx"
    );
}

pub fn wallet_response_send_failed_get_leader_aged_notes() {
    nomos_tracing::metric_counter_u64!(
        wallet_response_send_failed_total,
        1,
        kind = "get_leader_aged_notes"
    );
}
