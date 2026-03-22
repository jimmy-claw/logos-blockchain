//! C bindings for zone inscription and query operations.

use std::ffi::{CStr, CString, c_char};

use lb_common_http_client::CommonHttpClient;
use lb_core::mantle::{
    MantleTx, SignedMantleTx, Transaction as _,
    ledger::Tx as LedgerTx,
    ops::{
        Op, OpProof,
        channel::{ChannelId, MsgId, inscribe::InscriptionOp},
    },
};
use lb_key_management_system_keys::keys::{Ed25519Key, ZkKey};
use reqwest::Url;

/// Helper to decode a hex C string into a fixed-size byte array.
///
/// # Safety
///
/// `hex_ptr` must be a valid, non-null pointer to a null-terminated C string.
unsafe fn hex_to_bytes<const N: usize>(hex_ptr: *const c_char, name: &str) -> Option<[u8; N]> {
    let c_str = unsafe { CStr::from_ptr(hex_ptr) };
    let hex_str = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[{name}] Invalid UTF-8: {e}");
            return None;
        }
    };
    let bytes = match hex::decode(hex_str) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[{name}] Invalid hex: {e}");
            return None;
        }
    };
    match bytes.try_into() {
        Ok(arr) => Some(arr),
        Err(v) => {
            let v: Vec<u8> = v;
            eprintln!("[{name}] Expected {N} bytes, got {}", v.len());
            None
        }
    }
}

/// Helper to parse a C string as a URL.
///
/// # Safety
///
/// `ptr` must be a valid, non-null pointer to a null-terminated C string.
unsafe fn parse_url(ptr: *const c_char, name: &str) -> Option<Url> {
    let c_str = unsafe { CStr::from_ptr(ptr) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[{name}] Invalid UTF-8 in URL: {e}");
            return None;
        }
    };
    match Url::parse(s) {
        Ok(u) => Some(u),
        Err(e) => {
            eprintln!("[{name}] Invalid URL: {e}");
            None
        }
    }
}

/// Inscribe data to a zone channel via HTTP.
///
/// Returns the inscription ID (transaction hash) as a hex string.
/// The caller must free the result with [`free_zone_inscribe_result`].
/// Returns null on error.
///
/// # Safety
///
/// - All non-nullable pointer arguments must be valid.
/// - `data` must point to at least `data_len` bytes.
/// - `last_msg_id_hex` may be null for root/first inscription.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zone_inscribe(
    node_url: *const c_char,
    channel_id_hex: *const c_char,
    data: *const u8,
    data_len: usize,
    signing_key_hex: *const c_char,
    last_msg_id_hex: *const c_char,
) -> *mut c_char {
    if node_url.is_null() || channel_id_hex.is_null() || data.is_null() || signing_key_hex.is_null()
    {
        eprintln!("[zone_inscribe] Received a null pointer for a required argument.");
        return std::ptr::null_mut();
    }

    let Some(url) = (unsafe { parse_url(node_url, "zone_inscribe") }) else {
        return std::ptr::null_mut();
    };

    let Some(channel_id_bytes) =
        (unsafe { hex_to_bytes::<32>(channel_id_hex, "zone_inscribe/channel_id") })
    else {
        return std::ptr::null_mut();
    };
    let channel_id = ChannelId::from(channel_id_bytes);

    let Some(signing_key_bytes) =
        (unsafe { hex_to_bytes::<32>(signing_key_hex, "zone_inscribe/signing_key") })
    else {
        return std::ptr::null_mut();
    };
    let signing_key = Ed25519Key::from_bytes(&signing_key_bytes);

    let last_msg_id = if last_msg_id_hex.is_null() {
        MsgId::root()
    } else {
        let Some(msg_id_bytes) =
            (unsafe { hex_to_bytes::<32>(last_msg_id_hex, "zone_inscribe/last_msg_id") })
        else {
            return std::ptr::null_mut();
        };
        MsgId::from(msg_id_bytes)
    };

    let inscription_data = unsafe { std::slice::from_raw_parts(data, data_len) }.to_vec();

    // Build inscription transaction (logic from zone-sdk/src/sequencer.rs create_inscribe_tx)
    let signer = signing_key.public_key();
    let inscribe_op = InscriptionOp {
        channel_id,
        inscription: inscription_data,
        parent: last_msg_id,
        signer,
    };

    let ledger_tx = LedgerTx::new(vec![], vec![]);
    let inscribe_tx = MantleTx {
        ops: vec![Op::ChannelInscribe(inscribe_op)],
        ledger_tx,
        storage_gas_price: 0,
        execution_gas_price: 0,
    };

    let tx_hash = inscribe_tx.hash();
    let signature = signing_key.sign_payload(tx_hash.as_signing_bytes().as_ref());
    let ledger_tx_proof = match ZkKey::multi_sign(&[], tx_hash.as_ref()) {
        Ok(proof) => proof,
        Err(e) => {
            eprintln!("[zone_inscribe] Failed to create ledger proof: {e}");
            return std::ptr::null_mut();
        }
    };

    let signed_tx = SignedMantleTx {
        ops_proofs: vec![OpProof::Ed25519Sig(signature)],
        ledger_tx_proof,
        mantle_tx: inscribe_tx,
    };

    // Post transaction via HTTP
    let Ok(runtime) = tokio::runtime::Runtime::new() else {
        eprintln!("[zone_inscribe] Failed to create tokio runtime.");
        return std::ptr::null_mut();
    };

    let http_client = CommonHttpClient::new(None);
    if let Err(e) = runtime.block_on(http_client.post_transaction(url, signed_tx)) {
        eprintln!("[zone_inscribe] Failed to post transaction: {e}");
        return std::ptr::null_mut();
    }

    // Return inscription ID (tx hash) as hex string
    let tx_hash_hex = hex::encode(tx_hash.as_signing_bytes());
    match CString::new(tx_hash_hex) {
        Ok(c_str) => c_str.into_raw(),
        Err(e) => {
            eprintln!("[zone_inscribe] Failed to create result string: {e}");
            std::ptr::null_mut()
        }
    }
}

/// Free the result of [`zone_inscribe`].
///
/// # Safety
///
/// `result` must be a pointer returned by [`zone_inscribe`], or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_zone_inscribe_result(result: *mut c_char) {
    if !result.is_null() {
        drop(unsafe { CString::from_raw(result) });
    }
}

/// Query inscriptions from a zone channel by slot range.
///
/// Returns a JSON string: `[{"id":"hex","data":"hex","slot":123}, ...]`
/// The caller must free the result with [`free_zone_query_result`].
/// Returns null on error.
///
/// # Safety
///
/// All pointer arguments must be valid and non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zone_query_channel(
    node_url: *const c_char,
    channel_id_hex: *const c_char,
    from_slot: u64,
    limit: usize,
) -> *mut c_char {
    if node_url.is_null() || channel_id_hex.is_null() {
        eprintln!("[zone_query_channel] Received a null pointer.");
        return std::ptr::null_mut();
    }

    let Some(url) = (unsafe { parse_url(node_url, "zone_query_channel") }) else {
        return std::ptr::null_mut();
    };

    let Some(channel_id_bytes) =
        (unsafe { hex_to_bytes::<32>(channel_id_hex, "zone_query_channel/channel_id") })
    else {
        return std::ptr::null_mut();
    };
    let channel_id = ChannelId::from(channel_id_bytes);

    if limit == 0 {
        let empty = CString::new("[]").expect("static string");
        return empty.into_raw();
    }

    let Ok(runtime) = tokio::runtime::Runtime::new() else {
        eprintln!("[zone_query_channel] Failed to create tokio runtime.");
        return std::ptr::null_mut();
    };

    let http_client = CommonHttpClient::new(None);

    // Fetch blocks in the slot range and filter for channel inscriptions
    let to_slot = from_slot.saturating_add(limit.saturating_sub(1) as u64);
    let blocks = match runtime.block_on(http_client.get_blocks(url, from_slot, to_slot)) {
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("[zone_query_channel] Failed to fetch blocks: {e}");
            return std::ptr::null_mut();
        }
    };

    let mut results = Vec::new();
    for block in &blocks {
        let block_slot: u64 = block.header.slot.into();
        for tx in &block.transactions {
            for op in &tx.mantle_tx.ops {
                if let Op::ChannelInscribe(inscribe) = op {
                    if inscribe.channel_id == channel_id {
                        let id_bytes: [u8; 32] = *inscribe.id().as_ref();
                        results.push(serde_json::json!({
                            "id": hex::encode(id_bytes),
                            "data": hex::encode(&inscribe.inscription),
                            "slot": block_slot,
                        }));
                        if results.len() >= limit {
                            break;
                        }
                    }
                }
            }
            if results.len() >= limit {
                break;
            }
        }
        if results.len() >= limit {
            break;
        }
    }

    let json = serde_json::Value::Array(results).to_string();
    match CString::new(json) {
        Ok(c_str) => c_str.into_raw(),
        Err(e) => {
            eprintln!("[zone_query_channel] Failed to create result string: {e}");
            std::ptr::null_mut()
        }
    }
}

/// Free the result of [`zone_query_channel`].
///
/// # Safety
///
/// `result` must be a pointer returned by [`zone_query_channel`], or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_zone_query_result(result: *mut c_char) {
    if !result.is_null() {
        drop(unsafe { CString::from_raw(result) });
    }
}
