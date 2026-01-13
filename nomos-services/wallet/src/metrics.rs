use crate::WalletMsg;

const KIND_GET_BALANCE: &str = "get_balance";
const KIND_FUND_AND_SIGN_TX: &str = "fund_and_sign_tx";
const KIND_GET_LEADER_AGED_NOTES: &str = "get_leader_aged_notes";

pub fn wallet_request(msg: &WalletMsg) {
    let kind = match msg {
        WalletMsg::GetBalance { .. } => KIND_GET_BALANCE,
        WalletMsg::FundAndSignTx { .. } => KIND_FUND_AND_SIGN_TX,
        WalletMsg::GetLeaderAgedNotes { .. } => KIND_GET_LEADER_AGED_NOTES,
    };

    nomos_tracing::increase_counter_u64!(wallet_requests_total, 1, kind = kind);
}

pub fn wallet_response_send_failed_get_balance() {
    nomos_tracing::increase_counter_u64!(
        wallet_response_send_failed_total,
        1,
        kind = KIND_GET_BALANCE
    );
}

pub fn wallet_response_send_failed_fund_and_sign_tx() {
    nomos_tracing::increase_counter_u64!(
        wallet_response_send_failed_total,
        1,
        kind = KIND_FUND_AND_SIGN_TX
    );
}

pub fn wallet_response_send_failed_get_leader_aged_notes() {
    nomos_tracing::increase_counter_u64!(
        wallet_response_send_failed_total,
        1,
        kind = KIND_GET_LEADER_AGED_NOTES
    );
}
