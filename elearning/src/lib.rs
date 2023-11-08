pub mod application;
pub mod models;

use models::contract::{ELearningContract, ELearningContractExt, ELearningContractMetadata, ELearningStorageKey};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl ELearningContract {
  #[init]
  pub fn init() -> Self {
    let owner_id = env::signer_account_id();
    Self::new(
      owner_id,
      ELearningContractMetadata {
        spec: "elearning-1.0.0".to_string(),
        name: "elearing".to_string(),
        symbol: "EganTeam".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: ELearningContractMetadata) -> Self {
    Self {
      owner_id,
      user_address: env::signer_account_id(),
      metadata_contract: LazyOption::new(ELearningStorageKey::ContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
      subscriber_users: UnorderedSet::new(ELearningStorageKey::SubscriberUsers.try_to_vec().unwrap()),
      instructor_users: UnorderedSet::new(ELearningStorageKey::IntructorUsers.try_to_vec().unwrap()),
      all_course_id: UnorderedSet::new(ELearningStorageKey::AllCourseId.try_to_vec().unwrap()),
      courses_per_user: LookupMap::new(ELearningStorageKey::CoursesPerUser.try_to_vec().unwrap()),
      courses_per_instructor: LookupMap::new(ELearningStorageKey::CoursesPerInstructor.try_to_vec().unwrap()),
      course_metadata_by_id: LookupMap::new(ELearningStorageKey::CourseMetadataById.try_to_vec().unwrap()),
      certificate_per_user: LookupMap::new(ELearningStorageKey::CertificatesPerUser.try_to_vec().unwrap()),
      certificate_metadata_by_id: LookupMap::new(ELearningStorageKey::CertificateMetadataById.try_to_vec().unwrap()),
      skill_metadata_by_skill_id: LookupMap::new(ELearningStorageKey::SkillMetadataPerSkillId.try_to_vec().unwrap()),
    }
  }
}
