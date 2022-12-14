#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]

use codec::{self, Decode, Encode};
use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub use pallet::*;

pub mod traits;
pub use traits::*;
pub mod macros;

extern crate alloc;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{dispatch::Dispatchable, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Call: Parameter + Dispatchable + Encode + Decode + Rule;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn rules)]
    pub type Rules<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Box<<<T as Config>::Call as Rule>::CallRule>>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000_000_000)]
        pub fn new_rule(
            origin: OriginFor<T>,
            rule: <<T as Config>::Call as Rule>::CallRule,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let hash = (rule.get_id(), rule.clone())
                .using_encoded(<<T as frame_system::Config>::Hashing as Hash>::hash);

            Rules::<T>::insert(hash, rule);

            Self::deposit_event(Event::EvalResult { result: false });

            Ok(().into())
        }

        #[pallet::weight(1_000_000_000)]
        pub fn check_rule(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
            rule: Box<<<T as Config>::Call as Rule>::CallRule>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Self::deposit_event(Event::EvalResult {
                result: call.check_rule(*rule),
            });

            Ok(().into())
        }

        #[pallet::weight(1_000_000_000)]
        pub fn check_rule_hash(
            origin: OriginFor<T>,
            call: Box<<T as Config>::Call>,
            hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Self::deposit_event(Event::EvalResult {
                result: call.check_rule(*Rules::<T>::get(hash).ok_or(Error::<T>::HashNotFound)?),
            });

            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        HashNotFound,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        EvalResult { result: bool },
    }
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum V {
    String(Vec<u8>),
    Boolean(bool),
    Int(i64),
    Empty,

    Vec(Vec<Box<V>>),
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Eq(Box<Instruction>, Box<Instruction>),
    Not(Box<Instruction>, Box<Instruction>),
    Gt(Box<Instruction>, Box<Instruction>),
    Lt(Box<Instruction>, Box<Instruction>),
    And(Box<Instruction>, Box<Instruction>),
    Or(Box<Instruction>, Box<Instruction>),
    Data(V),
    Variable(Vec<u8>),
}
