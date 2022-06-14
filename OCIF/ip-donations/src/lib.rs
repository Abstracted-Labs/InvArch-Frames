//! # IP Donations FRAME Pallet.

//! Intellectual Property Donations
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//! This pallet demonstrates how to donate IP.
//!
//! ### Pallet Functions
//!
//! - `register` - 
//! - `donate` - 

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;
use sp-runtime::{PalletId, traits::AccountIdConversion};
use frame_support::traits::{OnUnbalanced, Imbalance};
use frame_system::pallet_prelude::OriginFor;
use primitives::DonationInfo;

const PALLET_ID: PalletId = PalletId(*b"Donation");

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Minimum amount that should be left on staker account after staking.
		#[pallet::constant]
		type MinimumRemainingAmount: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub type DonationInfoOf<T> = DonationInfo<
		<T as frame_system::Config>::AccountId,
		<T as pallet::Config>::Balance,
	>;

	#[pallet::storage]
	#[pallet::getter(fn donation_storage)]
	pub type DonationStorage<T: Config> = StorageMap<_, Blake2_128Concat, DonationInfoOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    DonationReceived(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		ImbalanceAbsorbed(NegativeImbalanceOf<T>, BalanceOf<T>),
	}

	// Errors for IP Donations
	#[pallet::error]
	pub enum Error<T> {
    /// Can not donate with zero value
		DonateWithNoValue,
	}

	// Dispatchable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Donate extrinsic which accepts the amount to be donated as parameter
		/// 
		/// Any user can call this function.
		/// However, caller have to have deposit amount
		#[pallet::weight(100_000 + T::DbWeight::get().reads_writes(1, 2))]
		fn donate(
			origin: OriginFor<T>,
			amount: BalanceOf<T>
	) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			let _ = T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath);

			// Ensure that donor has enough balance to donate.
			let free_balance = T::Currency::free_balance(&donor).saturating_sub(T::MinimumRemainingAmount::get());
			let available_balance = free_balance.saturating_sub(ledger.locked);
			let value_to_donate = value.min(available_balance);

			ensure!(
				value_to_donate > Zero::zero(),
				Error::<T>::DonateWithNoValue
			);

			DonationStorage::<T>::insert(&donor, &value_to_donate);

			Self::deposit_event(Event::DonationReceived(donor, value_to_donate, Self::pot()));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
    /// The account ID that holds the Donation's funds
    pub fn account_id() -> T::AccountId {
        PALLET_ID.into_account()
    }

    /// The Donations's balance
    fn pot() -> BalanceOf<T> {
        T::Currency::free_balance(&Self::account_id())
    }
	}

	type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Pallet<T> {
			fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
					let numeric_amount = amount.peek();

					// Must resolve into existing but better to be safe.
					let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

					Self::deposit_event(Event::ImbalanceAbsorbed(numeric_amount, Self::pot()));
			}
	}

	// TODO: Allocating Funds
	//
	// In order for the treasury to affect change with the funds it has collected it must be able to allocate those funds. Our IP Donation pallet abstracts the governance of where funds will be allocated to the rest of the runtime. Funds can be allocated by a root call to the `allocate` extrinsic. One good example of a governance mechanism for such decisions is Substrate's own

}