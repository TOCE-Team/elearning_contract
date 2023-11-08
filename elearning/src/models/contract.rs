use near_sdk::{
  collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
  json_types::Base64VecU8,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, CryptoHash, PanicOnDefault,
};

use crate::borsh::{self, BorshDeserialize, BorshSerialize};

use super::{
  certificate::{CertificateId, CertificateMetadata},
  course::{CourseId, CourseMetadata},
  skill::{SkillId, SkillMetadata},
};

/// The `ELearningContractMetadata` struct represents metadata for an e-learning contract.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ELearningContractMetadata {
  /// Specification associated with the e-learning contract.
  pub spec: String,

  /// Name of the e-learning contract.
  pub name: String,

  /// Symbol associated with the e-learning contract.
  pub symbol: String,

  /// Optional icon for the e-learning contract.
  pub icon: Option<String>,

  /// Optional base URI for the e-learning contract.
  pub base_uri: Option<String>,

  /// Optional reference string for the e-learning contract.
  pub reference: Option<String>,

  /// Optional hash of the reference, encoded in base64.
  pub reference_hash: Option<Base64VecU8>,
}

/// The `ELearningContract` struct represents an e-learning contract in the system.
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ELearningContract {
  /// Account ID of the owner of the contract.
  pub owner_id: AccountId,

  /// Pool address to stake
  pub user_address: AccountId,

  /// Metadata associated with the e-learning contract.
  pub metadata_contract: LazyOption<ELearningContractMetadata>,

  /// Storage all user_id of subscriber users -> For count all of users in the system
  pub subscriber_users: UnorderedSet<AccountId>,

  /// Storage all user_id of instructor users. -> For count all of instructors in the system
  pub instructor_users: UnorderedSet<AccountId>,

  /// Map of mentor users. -> For count all of mentors in the system
  // pub mentor_users: UnorderedMap<u32, AccountId>,

  /// Map of `JsonUser` metadata by user ID.

  /// All course id
  pub all_course_id: UnorderedSet<CourseId>,

  /// Map of course sets by user ID.
  pub courses_per_user: LookupMap<AccountId, UnorderedSet<CourseId>>,

  /// Map of course sets by Instructors
  pub courses_per_instructor: LookupMap<AccountId, UnorderedSet<CourseId>>,

  /// Map of `CourseMetadata` by course ID.
  pub course_metadata_by_id: LookupMap<CourseId, CourseMetadata>,

  /// Map of certificate sets by user ID.
  pub certificate_per_user: LookupMap<AccountId, UnorderedSet<CertificateId>>,

  /// Map of `CertificateMetadata` by certificate ID.
  pub certificate_metadata_by_id: LookupMap<CertificateId, CertificateMetadata>,

  /// Map of SkillMetadata by SkillId
  pub skill_metadata_by_skill_id: LookupMap<SkillId, UnorderedSet<SkillMetadata>>,

  // Storage all user_id of subscriber users -> For count all of users in the system

  // Map of PoolMetadata by PoolId



}

/// The `ELearningContractStorageKey` enum represents keys for different persistent collections in the contract storage.
#[derive(BorshSerialize)]
pub enum ELearningStorageKey {
  ContractMetadata,
  SubscriberUsers,
  IntructorUsers,
  // MentorUsers,
  UserMetadataById,
  AllCourseId,
  CoursesPerUser,
  CourseMetadataById,
  CertificatesPerUser,
  CertificateMetadataById,
  CertificatePerUserInner { account_id_hash: CryptoHash },
  CoursesPerInstructor,
  CoursesPerInstructorInner { instructor_id_hash: CryptoHash },
  SkillMetadataPerSkillId,
  SkillMetadataPerSkillIdInner { skill_id_hash: CryptoHash },
  AllPoolId,
  PoolMetadataByPoolId,
  AllComboId,
  ComboMetadataByComboId,
  AllMentoringId,
  MentoringMetadataByMentoringId,
}
