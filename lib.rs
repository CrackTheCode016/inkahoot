#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod quiz {
    use ink::env::hash::{Blake2x256, CryptoHash, HashOutput};
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::Encode;

    #[derive(scale::Decode, scale::Encode, Debug, Clone, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Question {
        question: String,
        answer: [u8; 32],
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum PowerLevel {
        Educator,
        User,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Error {
        WrongAnswer,
        QuestionDoesntExist,
        InvalidPowerLevel,
        InvalidCaller,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Quiz {
        /// Owner of quiz
        owner: AccountId,
        /// Stores a single `bool` value on the storage.
        questions: Vec<Question>,
        /// Mapping of users that register to use this contract
        actors: Mapping<AccountId, PowerLevel>,
    }

    impl Quiz {
        /// Creates a new quiz contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner: AccountId = Self::env().caller();
            let mut actors = Mapping::default();
            // The creator of the contract is the "Owner"
            actors.insert(owner, &PowerLevel::Educator);
            let questions = Vec::new();
            Self {
                questions,
                actors,
                owner,
            }
        }

        #[ink(message)]
        pub fn add_question(&mut self, question: String, answer: String) -> Result<(), Error> {
            let caller = Self::env().caller();
            Self::ensure_powerlevel(&self, caller, PowerLevel::Educator)?;
            let answer_hash = Self::hash::<Blake2x256, String>(answer);
            self.questions.push(Question {
                question,
                answer: answer_hash,
            });
            return Ok(());
        }

        #[ink(message)]
        pub fn add_educator(&mut self, educator: AccountId) -> Result<(), Error> {
            let caller = Self::env().caller();
            self.ensure_contract_owner(caller)?;
            self.actors.insert(educator, &PowerLevel::Educator);
            Err(Error::InvalidCaller)
        }

        /// Simply returns a question (if it exists)
        #[ink(message)]
        pub fn get(&self, index: u32) -> Result<Question, Error> {
            let question = self.questions.get(index as usize);
            if let Some(valid_question) = question {
                return Ok(valid_question.clone());
            }
            Err(Error::QuestionDoesntExist)
        }

        /// Check if an answer is correct
        #[ink(message)]
        pub fn check_answer(&self, index: u32, attempt: String) -> Result<bool, Error> {
            let question = Self::get(&self, index)?;
            let answer_hash = Self::hash::<Blake2x256, String>(attempt);
            if question.answer == answer_hash {
                return Ok(true);
            }
            Err(Error::WrongAnswer)
        }

        /// Hashes a value with any supported hashing algos
        fn hash<S: CryptoHash + HashOutput, T: Encode>(entity: T) -> <S as HashOutput>::Type {
            let mut hash = <<S as HashOutput>::Type as Default>::default();
            <S as CryptoHash>::hash(&entity.encode(), &mut hash);
            return hash;
        }

        fn ensure_powerlevel(&self, id: AccountId, level: PowerLevel) -> Result<(), Error> {
            if let Some(power_level) = self.actors.get(id) {
                if power_level == level {
                    return Ok(());
                }
                return Err(Error::InvalidPowerLevel);
            }
            return Err(Error::InvalidCaller);
        }

        fn ensure_contract_owner(&self, id: AccountId) -> Result<(), Error> {
            if id != self.owner {
                return Err(Error::InvalidCaller);
            }
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let quiz = Quiz::new();
            assert!(quiz.get(0).is_err());
        }

        /// We test if creating a new contract with questions works.
        #[ink::test]
        fn add_questions() {
            let mut quiz = Quiz::new();
            quiz.add_question(String::from("What color is the sky?"), String::from("Blue"))
                .unwrap();
            assert!(quiz.get(0).is_ok());
            assert_eq!(
                quiz.get(0).unwrap().question,
                String::from("What color is the sky?")
            );
        }

        /// We test if providing the correct answer works.
        #[ink::test]
        fn correct_answer_works() {
            let answer = String::from("Blue");
            let mut quiz = Quiz::new();
            quiz.add_question(String::from("What color is the sky?"), answer.clone())
                .unwrap();
            assert!(quiz.check_answer(0, answer.clone()).is_ok());
            assert_eq!(quiz.check_answer(0, answer).unwrap(), true);
        }

        /// We test if the wrong answer should fail.
        #[ink::test]
        fn wrong_answer_should_fail() {
            let wrong_answer = String::from("Green XD");
            let mut quiz = Quiz::new();
            quiz.add_question(String::from("What color is the sky?"), String::from("Blue"))
                .unwrap();
            assert!(quiz.get(0).is_ok());
            assert!(quiz.check_answer(0, wrong_answer).is_err());
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {}
}
