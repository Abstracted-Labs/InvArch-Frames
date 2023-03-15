#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Get;
use sp_std::convert::TryInto;

mod traits;

use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use frame_support::{
    pallet_prelude::DispatchResult,
    traits::fungibles::{Balanced, CreditOf, Inspect, Unbalanced},
};
use orml_traits::{Happened, MultiCurrency, MultiCurrencyExtended};
pub use pallet::*;
use pallet_asset_tx_payment::{HandleCredit, OnChargeAssetTransaction};
use scale_info::TypeInfo;
use sp_arithmetic::FixedPointOperand;
use sp_runtime::{
    traits::{DispatchInfoOf, One, PostDispatchInfoOf, Saturating, Zero},
    transaction_validity::{InvalidTransaction, TransactionValidityError},
    FixedU128, ModuleError,
};
pub use traits::FeeAssets;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{FullCodec, MaxEncodedLen};
    use core::fmt::Debug;
    use frame_support::{pallet_prelude::*, sp_runtime::traits::SignedExtension};
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use sp_std::{vec, vec::Vec};

    pub type BalanceOf<T> = <<T as Config>::Currencies as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_asset_tx_payment::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Currencies: MultiCurrency<Self::AccountId, CurrencyId = Self::AssetId>
            + MultiCurrencyExtended<Self::AccountId>
            + Unbalanced<Self::AccountId>
            + Balanced<Self::AccountId>
            + Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = BalanceOf<Self>>;

        type AssetId: FullCodec
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + Default
            + Eq
            + TypeInfo
            + MaxEncodedLen;

        type Assets: FeeAssets<Self::AssetId, <Self as frame_system::Config>::RuntimeCall>;

        // type HandleFee: HandleFee<Self::AccountId, Self::AssetId, BalanceOf<Self>>;

        type HandleCredit: HandleCredit<Self::AccountId, Self::Currencies>;
    }

    #[pallet::storage]
    #[pallet::getter(fn preferred_asset)]
    pub type PreferredAsset<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::AssetId>;

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(100_000_000)]
        pub fn set_preferred_asset(origin: OriginFor<T>, asset: T::Assets) -> DispatchResult {
            let o = ensure_signed(origin)?;

            PreferredAsset::<T>::insert(o, asset.get_asset_id());

            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub struct LiquidityInfo<Balance, AssetId> {
    pub fee: Balance,
    pub asset_id: AssetId,
}

/// Handler for dealing with fees
pub trait HandleFee<AccountId, AssetId, Balance> {
    fn handle_fee(currency: AssetId, amount: Balance) -> DispatchResult;
}

impl<T: Config> OnChargeAssetTransaction<T> for Pallet<T> {
    type AssetId = T::AssetId;
    type LiquidityInfo = CreditOf<T::AccountId, T::Currencies>;

    type Balance =
        <T::Currencies as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Withdraw the predicted fee from the transaction origin.
    ///
    /// Note: The `fee` already includes the `tip`.
    fn withdraw_fee(
        who: &T::AccountId,
        call: &T::RuntimeCall,
        _dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
        asset_id: Self::AssetId,
        fee: Self::Balance,
        _tip: Self::Balance,
    ) -> Result<Self::LiquidityInfo, TransactionValidityError> {
        //if fee.is_zero() {
        //    return Ok(None);
        //}

        let asset_id = PreferredAsset::<T>::get(who)
            .map(|a| T::Assets::can_be_used(a, call))
            .unwrap_or(T::Assets::default_asset());

        //  match T::Currencies::withdraw(asset_id.clone(), who, fee) {
        //      Ok(()) => Ok(Some(LiquidityInfo {
        //          fee,
        //          asset_id: asset_id.into(),
        //      })),
        //      Err(_) => Err(InvalidTransaction::Payment.into()),
        //  }

        <T::Currencies as Balanced<T::AccountId>>::withdraw(asset_id, who, fee)
            .map_err(|_| TransactionValidityError::from(InvalidTransaction::Payment))
    }

    /// Since the predicted fee might have been too high, parts of the fee may
    /// be refunded.
    ///
    /// Note: The `fee` already includes the `tip`.
    fn correct_and_deposit_fee(
        who: &T::AccountId,
        _dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
        _post_info: &PostDispatchInfoOf<T::RuntimeCall>,
        corrected_fee: Self::Balance,
        tip: Self::Balance,
        paid: Self::LiquidityInfo,
    ) -> Result<(), TransactionValidityError> {
        // if let Some(paid) = already_withdrawn {
        // let asset_id = paid.asset_id;
        // let refund = paid.fee.saturating_sub(corrected_fee);
        // let fee = corrected_fee.saturating_sub(tip);

        // refund to the account that paid the fees
        // T::Currencies::deposit(asset_id.clone(), who, refund)
        //    .map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;

        // deposit the fee
        // T::HandleFee::handle_fee(asset_id.into(), fee + tip)
        //    .map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;
        // }

        let (final_fee, refund) = paid.split(corrected_fee);

        let _ = <T::Currencies as Balanced<T::AccountId>>::resolve(who, refund);

        T::HandleCredit::handle_credit(final_fee);

        Ok(())
    }
}
