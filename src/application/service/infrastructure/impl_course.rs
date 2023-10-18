use crate::{models::{
  certificate::CertificateFeatures,
  contract::{ELearningContract, ELearningContractExt},
  course::{CourseFeatures, CourseId, CourseMetadata},
  skill::{SkillFeatures, SkillId, WrapSkill},
}, application::repository::convert_coure_title_to_cousrse_id};
use near_sdk::{env, json_types::U128, near_bindgen, Balance, AccountId};
use std::collections::HashMap;

#[near_bindgen]
impl CourseFeatures for ELearningContract {
  fn create_course(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
    is_instructor: bool,
  ) -> CourseMetadata {
    let instructor_id = env::signer_account_id();
    assert!(is_instructor,  "You aren't an instructor, You need register & upload your resume to become a instructor!");
    
    let course_id = convert_coure_title_to_cousrse_id(&title, instructor_id.to_string());

    let price: Balance = price.into();

    let mut initial_instructor: HashMap<AccountId, u32> = HashMap::new();
    initial_instructor.insert(instructor_id.clone(), 10000);

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

    return course_metadata;
  }

  // TODO: More Requirement to check

}
