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
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseResult};
use std::{collections::HashMap, ptr::null};

#[near_bindgen]
impl CourseFeatures for ELearningContract {
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
      .then(Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).test_create_course());
  }

  #[private]
  fn test_create_course(&mut self) -> u128 {
    let result = match env::promise_result(0) {
      PromiseResult::NotReady => env::abort(),
      PromiseResult::Successful(value) => {
        if let Ok(refund) = near_sdk::serde_json::from_slice::<U128>(&value) {
          refund.0
          // If we can't properly parse the value, the original amount is returned.
        } else {
          U128(2).into()
        }
      },
      PromiseResult::Failed => U128(2).into(),
    };

    return result;
  }

  // TODO: More Requirement to check
}
