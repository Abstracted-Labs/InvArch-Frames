#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]

use codec::{self, Decode, Encode};
use frame_system::ensure_signed;
use sp_std::prelude::*;

pub use pallet::*;

pub mod traits;
pub use traits::*;
pub mod macros;

extern crate alloc;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use core::fmt::Display;
    use frame_support::{dispatch::Dispatchable, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32BitUnsigned;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Call: Parameter + Dispatchable + Encode + Decode + Rule;

        type RulesetId: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + Display
            + MaxEncodedLen
            + Clone;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn ruleset)]
    pub type Ruleset<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet::Config>::RulesetId,
        Box<<<T as pallet::Config>::Call as Rule>::RuleSet>,
    >;

    #[pallet::error]
    pub enum Error<T> {
        HashNotFound,
        SetNotFound,
        SetAlreadyExists,

        Error,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        EvalResult { result: bool },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000_000_000)]
        pub fn new_rule(
            origin: OriginFor<T>,
            id: <T as pallet::Config>::RulesetId,
            rule: <<T as Config>::Call as Rule>::CallRule,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Ruleset::<T>::try_mutate(id, |set| -> DispatchResult {
                let mut old_set = set.take().ok_or(Error::<T>::SetNotFound)?;

                old_set.add_rule(rule);

                *set = Some(old_set);

                Ok(())
            })?;

            Ok(())
        }

        #[pallet::weight(1_000_000_000)]
        pub fn initialize_rule_set(
            origin: OriginFor<T>,
            id: <T as pallet::Config>::RulesetId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Ruleset::<T>::try_mutate(id, |set| -> DispatchResult {
                if set.take().is_some() {
                    return Err(Error::<T>::Error.into());
                }

                *set = Some(Box::new(
                    <<T as pallet::Config>::Call as Rule>::RuleSet::new(),
                ));

                Ok(())
            })?;

            Ok(())
        }

        #[pallet::weight(1_000_000_000)]
        pub fn check_rule(
            origin: OriginFor<T>,
            id: <T as pallet::Config>::RulesetId,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let set = Ruleset::<T>::get(&id).ok_or(Error::<T>::SetNotFound)?;

            Self::deposit_event(Event::EvalResult {
                result: call.check_rule(&*set),
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn initialize_set(id: <T as pallet::Config>::RulesetId) -> DispatchResult {
            Ruleset::<T>::try_mutate(id, |set| -> DispatchResult {
                if set.take().is_some() {
                    return Err(Error::<T>::SetAlreadyExists.into());
                }

                *set = Some(Box::new(
                    <<T as pallet::Config>::Call as Rule>::RuleSet::new(),
                ));

                Ok(())
            })
        }
    }
}
