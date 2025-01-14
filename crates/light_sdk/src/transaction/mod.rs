use std::str::FromStr;

use borsh::BorshSerialize;
use namada_sdk::tx::{Section, Signature, Signer, Tx};
use namada_sdk::types::chain::ChainId;
use namada_sdk::types::hash::Hash;
use namada_sdk::types::key::common;
use namada_sdk::types::time::DateTimeUtc;

pub mod account;
pub mod bridge;
pub mod governance;
pub mod ibc;
pub mod pgf;
pub mod pos;
pub mod transfer;
pub mod wrapper;

/// Generic arguments required to construct a transaction
pub struct GlobalArgs {
    pub expiration: Option<DateTimeUtc>,
    pub code_hash: Hash,
    pub chain_id: ChainId,
}

pub(in crate::transaction) fn build_tx(
    GlobalArgs {
        expiration,
        code_hash,
        chain_id,
    }: GlobalArgs,
    data: impl BorshSerialize,
    code_tag: String,
) -> Tx {
    let mut inner_tx = Tx::new(chain_id, expiration);

    inner_tx.header.timestamp =
        DateTimeUtc::from_str("2000-01-01T00:00:00Z").unwrap();
    inner_tx.add_code_from_hash(code_hash, Some(code_tag));
    inner_tx.add_data(data);

    inner_tx
}

pub(in crate::transaction) fn get_sign_bytes(tx: &Tx) -> Vec<Hash> {
    vec![tx.raw_header_hash()]
}

pub(in crate::transaction) fn attach_raw_signatures(
    mut tx: Tx,
    signer: common::PublicKey,
    signature: common::Signature,
) -> Tx {
    tx.protocol_filter();
    tx.add_section(Section::Signature(Signature {
        targets: vec![tx.raw_header_hash()],
        signer: Signer::PubKeys(vec![signer]),
        signatures: [(0, signature)].into_iter().collect(),
    }));
    tx
}
