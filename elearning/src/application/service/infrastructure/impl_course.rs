use crate::{
  application::repository::convert_coure_title_to_cousrse_id,
  models::{
    certificate::CertificateFeatures,
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseFeatures, CourseId, CourseMetadata},
    skill::{SkillFeatures, SkillId, WrapSkill},
    user_request::external::{cross_user, GAS_FOR_CHECK_RESULT},
  },
};
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseError, PromiseResult};
use std::{collections::HashMap, ptr::null};

#[near_bindgen]
impl CourseFeatures for ELearningContract {
  fn update_user_ct_address(&mut self, user_address: AccountId) {
    assert!(env::signer_account_id() == self.owner_id);
    self.user_address = user_address;
  }

  fn check_user_ct_address(&mut self) -> AccountId {
    return self.user_address.to_owned();
  }

  fn test_cross_call(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
  ) {
    cross_user::ext(self.user_address.to_owned())
      .with_static_gas(GAS_FOR_CHECK_RESULT)
      .check_instructor()
      .then(Self::ext(self.user_address.to_owned()).with_static_gas(GAS_FOR_CHECK_RESULT).change_greeting_callback());
  }

  #[private]
  fn change_greeting_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) -> bool {
    // Return whether or not the promise succeeded using the method outlined in external.rs
    if call_result.is_err() {
      env::log_str("set_greeting failed...");
      return false;
    } else {
      env::log_str("set_greeting was successful!");
      return true;
    }
  }

  // TODO: More Requirement to check
}
