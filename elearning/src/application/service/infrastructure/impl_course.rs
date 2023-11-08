use crate::{
  application::repository::convert_coure_title_to_cousrse_id,
  models::{
    certificate::CertificateFeatures,
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseFeatures, CourseId, CourseMetadata},
    skill::{SkillFeatures, SkillId, WrapSkill},
    user_request::external::{cross_user, GAS_FOR_CHECK_RESULT, GAS_FOR_CROSS_CALL},
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

  fn create_course_by_instructor(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
  ) {
    let user_id = env::signer_account_id();

    cross_user::ext(self.user_address.to_owned()).with_static_gas(GAS_FOR_CROSS_CALL).check_instructor(user_id).then(
      Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).update_course_callback(
        title,
        description,
        media,
        price,
        skills,
      ),
    );
  }

  #[private]
  fn update_course_callback(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
  ) {
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

    let user_id: AccountId = env::signer_account_id();

    let price: Balance = price.into();
    let mut initial_instructor: HashMap<AccountId, u32> = HashMap::new();
    initial_instructor.insert(user_id.clone(), 10000);
    if result == 1 {
      let course_id = convert_coure_title_to_cousrse_id(&title, user_id.to_string());
      let course_metadata = CourseMetadata {
        course_id: course_id.clone(),
        title,
        skills,
        price,
        media,
        description,
        instructor_id: initial_instructor,
        created_at: env::block_timestamp_ms(),
        update_at: env::block_timestamp_ms(),
        students_completed: HashMap::new(),
        students_studying_map: HashMap::new(),
        rating: 0,
        rating_count: 0,
        content: "".to_string(),
        consensus: HashMap::new(),
      };
      self.course_metadata_by_id.insert(&course_id, &course_metadata);
      self.all_course_id.insert(&course_id);

      // Storage in user data
    }
  }

  // TODO: More Requirement to check
}
