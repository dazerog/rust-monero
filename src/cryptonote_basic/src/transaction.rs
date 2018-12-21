use crate::transaction_prefix::TransactionPrefix;
use std::sync::atomic::AtomicBool;
use xmr_crypto::crypto::Signature;
use xmr_crypto::hash::Hash;

pub struct Transaction {
    prefix: TransactionPrefix,
    hash_valid: AtomicBool,
    blob_size_valid: AtomicBool,

    signatures: Vec<Vec<Signature>>,
    //TODO  ringct
    //rct::rctSig rct_signatures;

    hash: Hash,
    blob_size: u8
}