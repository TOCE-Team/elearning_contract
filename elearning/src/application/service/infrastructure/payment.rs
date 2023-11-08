use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, near_bindgen,
  serde::{Deserialize, Serialize},
  Balance, Promise,
};

use crate::{
  application::repository::convert_to_yocto,
  models::{
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseId, EnumCourse},
    user_request::external::{cross_user, GAS_FOR_CROSS_CALL},
  },
};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WrapComboHash {
  /// Course in combo
  pub course_id: CourseId,

  pub encode_check: String,
}

pub trait Payment {
  fn payment_course(&mut self, course_id: CourseId, encode_check: String);

  fn internal_tranfer_course(&mut self, course_id: CourseId, price: Balance, encode_check: String);
}

#[near_bindgen]
impl Payment for ELearningContract {
  #[payable]
  fn payment_course(&mut self, course_id: CourseId, encode_check: String) {
    // Check course has exists
    let user_id = env::signer_account_id();
    let course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    // assert!(self.internal_check_subscriber_has_course(&user_id, &course_id), "You already have this course!");
    let amount_deposit = env::attached_deposit();
    assert!(amount_deposit >= convert_to_yocto(course.price), "You do not deposit enough money");
    self.internal_tranfer_course(course_id, course.price, encode_check);
  }

  #[private]
  fn internal_tranfer_course(&mut self, course_id: CourseId, price: Balance, encode_check: String) {
    let user_id = env::signer_account_id();
    let price = convert_to_yocto(price);
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    // Plus 1 student to course owner
    for i in course.instructor_id.keys() {
      let tranfer_percent = course.instructor_id.get(i).unwrap();
      let convert_to_subtract = (*tranfer_percent) as u128;
      Promise::new(i.clone()).transfer(price / 10_000 * convert_to_subtract);
    }

    course.students_studying_map.insert(user_id.clone(), encode_check.to_string());
    self.course_metadata_by_id.insert(&course_id, &course);

    cross_user::ext(self.user_address.to_owned()).with_static_gas(GAS_FOR_CROSS_CALL).update_course_by_user(course_id);
    
  }
}
