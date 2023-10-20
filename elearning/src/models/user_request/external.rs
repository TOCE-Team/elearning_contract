use near_sdk::{ext_contract, Gas, PromiseOrValue, Promise, json_types::U128, AccountId};

use crate::models::skill::SkillId;

pub const GAS_FOR_CHECK_RESULT: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);

/// Cross call pool contract and storage pool id
#[ext_contract(cross_user)]
pub trait CrossCall {
  /// Cross call pool contract and storage pool id
  fn check_instructor(&mut self) -> PromiseOrValue<bool>;
}
