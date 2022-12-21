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

// Temporary.
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{TrailingZeroInput, Zero};

/// Generates an `AccountId` using an `IpId` as the seed + the PalletId as salt.
pub fn derive_ips_account<T: frame_system::Config, IpId, AccountId: Decode + Clone>(
    ips_id: IpId,
    original_caller: Option<&AccountId>,
) -> AccountId
where
    (T::Hash, IpId): Encode,
    (T::Hash, IpId, AccountId): Encode,
{
    let entropy = if let Some(original_caller) = original_caller {
        (
            frame_system::Pallet::<T>::block_hash(T::BlockNumber::zero()),
            ips_id,
            original_caller.clone(),
        )
            .using_encoded(blake2_256)
    } else {
        (
            frame_system::Pallet::<T>::block_hash(T::BlockNumber::zero()),
            ips_id,
        )
            .using_encoded(blake2_256)
    };

    Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
        .expect("infinite length input; no invalid inputs for type; qed")
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::Dispatchable, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::DispatchResultWithInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Call: Parameter + Dispatchable + Encode + Decode + Rule;

        type Id: Parameter
            + Member
            + PartialOrd
            + PartialEq
            + Default
            + Copy
            + MaxEncodedLen
            + Clone;
    }

    pub type RulesetId<T> = (<T as pallet::Config>::Id, <T as pallet::Config>::Id);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn ruleset)]
    pub type Ruleset<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        RulesetId<T>,
        Box<<<T as pallet::Config>::Call as Rule>::RuleSet>,
    >;

    #[pallet::error]
    pub enum Error<T> {
        HashNotFound,
        SetNotFound,
        SetAlreadyExists,
        NoPermission,

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
            id: RulesetId<T>,
            rule: <<T as Config>::Call as Rule>::CallRule,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            ensure!(
                origin
                    == derive_ips_account::<
                        T,
                        <T as pallet::Config>::Id,
                        <T as frame_system::Config>::AccountId,
                    >(id.0, None),
                Error::<T>::NoPermission
            );

            Ruleset::<T>::try_mutate(id, |set| -> DispatchResult {
                let mut old_set = set.take().ok_or(Error::<T>::SetNotFound)?;

                old_set.add_rule(rule);

                *set = Some(old_set);

                Ok(())
            })?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn initialize_rule_set(id: RulesetId<T>) -> DispatchResult {
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

        pub fn check_rule(
            id: RulesetId<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithInfo<bool> {
            let set = Ruleset::<T>::get(&id).ok_or(Error::<T>::SetNotFound)?;

            Ok(call.check_rule(&*set))
        }
    }
}
