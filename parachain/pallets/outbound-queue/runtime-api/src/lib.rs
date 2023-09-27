// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

use snowbridge_core::outbound::{FeeAmount, Message, SubmitError};
use snowbridge_outbound_queue_merkle_tree::MerkleProof;

sp_api::decl_runtime_apis! {
	pub trait OutboundQueueApi
	{
		fn prove_message(leaf_index: u64) -> Option<MerkleProof>;

		fn compute_fee_reward_by_command_index(command_index: u8) -> Result<(FeeAmount, FeeAmount), SubmitError>;

		fn compute_fee_reward(_message: &Message) -> Result<(FeeAmount, FeeAmount), SubmitError>;
	}
}
