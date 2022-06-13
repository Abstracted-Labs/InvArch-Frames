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
use sp-runtime::{ModuleId, traits::AccountIdConversion};
use frame_support::traits::{OnUnbalanced, Imbalance};

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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    // TODO:
	}

	// Errors for IP Donations
	#[pallet::error]
	pub enum Error<T> {
    // TODO:
	}

	// Dispatchable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Donate extrinsic which accepts the amount to be donated as parameter
		/// 
		fn donate(
			origin,
			amount: BalanceOf<T>
	) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			let _ = T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath);

			Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
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

	impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
			fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
					let numeric_amount = amount.peek();

					// Must resolve into existing but better to be safe.
					let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

					Self::deposit_event(RawEvent::ImbalanceAbsorbed(numeric_amount, Self::pot()));
			}
	}

	// TODO: Allocating Funds
	//
	// In order for the treasury to affect change with the funds it has collected it must be able to allocate those funds. Our IP Donation pallet abstracts the governance of where funds will be allocated to the rest of the runtime. Funds can be allocated by a root call to the `allocate` extrinsic. One good example of a governance mechanism for such decisions is Substrate's own

}