#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::comparison_chain)]

use codec::MaxEncodedLen;
use frame_support::{
    ensure,
    pallet_prelude::*,
    traits::{
        tokens::{
            fungibles, DepositConsequence, Fortitude, Precision, Preservation, Provenance,
            WithdrawConsequence,
        },
        ConstBool, DefensiveSaturating,
    },
};
use frame_system::{ensure_signed, pallet_prelude::*};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{
        AtLeast32BitUnsigned, CheckedAdd, CheckedSub, IdentifyAccount, MaybeSerializeDeserialize,
        Member, Saturating, StaticLookup, Zero,
    },
    ArithmeticError, DispatchError, DispatchResult, FixedPointOperand, TokenError,
};
use sp_std::prelude::*;

mod weights;
pub use weights::WeightInfo;

pub use module::*;

#[frame_support::pallet]
pub mod module {

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type AccountId: Parameter
            + Member
            + Ord
            + MaxEncodedLen
            + From<<Self as frame_system::Config>::AccountId>
            + IdentifyAccount<AccountId = <Self as frame_system::Config>::AccountId>;

        type Lookup: sp_runtime::traits::StaticLookup<Target = <Self as Config>::AccountId>;

        /// The balance type
        type Balance: Parameter
            + Member
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + FixedPointOperand;

        /// The currency ID type
        type CurrencyId: Parameter
            + Member
            + Copy
            + MaybeSerializeDeserialize
            + Ord
            + TypeInfo
            + MaxEncodedLen;

        /// Weight information for extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The balance is too low
        BalanceTooLow,
        /// Cannot convert Amount into Balance type
        AmountIntoBalanceFailed,
        /// Failed because liquidity restrictions due to locking
        LiquidityRestrictions,
        /// Transfer/payment would kill account
        KeepAlive,
        /// Beneficiary account must pre-exist
        DeadAccount,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An account was created with some free balance.
        Endowed {
            currency_id: T::CurrencyId,
            who: <T as module::Config>::AccountId,
            amount: T::Balance,
        },
        /// Transfer succeeded.
        Transfer {
            currency_id: T::CurrencyId,
            from: <T as module::Config>::AccountId,
            to: <T as module::Config>::AccountId,
            amount: T::Balance,
        },
        /// A balance was set by root.
        BalanceSet {
            currency_id: T::CurrencyId,
            who: <T as module::Config>::AccountId,
            balance: T::Balance,
        },
        /// The total issuance of an currency has been set
        TotalIssuanceSet {
            currency_id: T::CurrencyId,
            amount: T::Balance,
        },
        /// Some balances were withdrawn (e.g. pay for transaction fee)
        Withdrawn {
            currency_id: T::CurrencyId,
            who: <T as module::Config>::AccountId,
            amount: T::Balance,
        },
        /// Some balances were slashed (e.g. due to mis-behavior)
        Slashed {
            currency_id: T::CurrencyId,
            who: <T as module::Config>::AccountId,
            free_amount: T::Balance,
            reserved_amount: T::Balance,
        },
        /// Deposited some balance into an account
        Deposited {
            currency_id: T::CurrencyId,
            who: <T as module::Config>::AccountId,
            amount: T::Balance,
        },
    }

    /// The total issuance of a token type.
    #[pallet::storage]
    #[pallet::getter(fn total_issuance)]
    pub type TotalIssuance<T: Config> =
        StorageMap<_, Twox64Concat, T::CurrencyId, T::Balance, ValueQuery>;

    /// The balance of a token type under an account.
    ///
    /// NOTE: If the total is ever zero, decrease account ref account.
    ///
    /// NOTE: This is only used in the case that this module is used to store
    /// balances.
    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub type Accounts<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as module::Config>::AccountId,
        Twox64Concat,
        T::CurrencyId,
        T::Balance,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn accounts_by_currency)]
    pub type AccountsByCurrency<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::CurrencyId,
        Blake2_128Concat,
        <T as module::Config>::AccountId,
        (),
    >;

    #[pallet::storage]
    #[pallet::getter(fn is_frozen)]
    pub type Frozen<T: Config> =
        StorageMap<_, Twox64Concat, T::CurrencyId, bool, ValueQuery, ConstBool<true>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfer some liquid free balance to another account.
        ///
        /// `transfer` will set the `FreeBalance` of the sender and receiver.
        /// It will decrease the total issuance of the system by the
        /// `TransferFee`. If the sender's account is below the existential
        /// deposit as a result of the transfer, the account will be reaped.
        ///
        /// The dispatch origin for this call must be `Signed` by the
        /// transactor.
        ///
        /// - `dest`: The recipient of the transfer.
        /// - `currency_id`: currency type.
        /// - `amount`: free balance amount to tranfer.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: <<T as Config>::Lookup as StaticLookup>::Source,
            currency_id: T::CurrencyId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = <T as Config>::Lookup::lookup(dest)?;
            Self::do_transfer(
                currency_id,
                &<T as Config>::AccountId::from(from),
                &to,
                amount,
            )
        }

        /// Transfer all remaining balance to the given account.
        ///
        /// NOTE: This function only attempts to transfer _transferable_
        /// balances. This means that any locked, reserved, or existential
        /// deposits (when `keep_alive` is `true`), will not be transferred by
        /// this function. To ensure that this function results in a killed
        /// account, you might need to prepare the account by removing any
        /// reference counters, storage deposits, etc...
        ///
        /// The dispatch origin for this call must be `Signed` by the
        /// transactor.
        ///
        /// - `dest`: The recipient of the transfer.
        /// - `currency_id`: currency type.
        /// - `keep_alive`: A boolean to determine if the `transfer_all`
        ///   operation should send all of the funds the account has, causing
        ///   the sender account to be killed (false), or transfer everything
        ///   except at least the existential deposit, which will guarantee to
        ///   keep the sender account alive (true).
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::transfer_all())]
        pub fn transfer_all(
            origin: OriginFor<T>,
            dest: <<T as Config>::Lookup as StaticLookup>::Source,
            currency_id: T::CurrencyId,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = <T as Config>::Lookup::lookup(dest)?;
            let preservation = Preservation::Expendable;

            let from = <T as Config>::AccountId::from(from);

            let reducible_balance = <Self as fungibles::Inspect<
                <T as module::Config>::AccountId,
            >>::reducible_balance(
                currency_id, &from, preservation, Fortitude::Polite
            );
            <Self as fungibles::Mutate<_>>::transfer(
                currency_id,
                &from,
                &to,
                reducible_balance,
                preservation,
            )
            .map(|_| ())
        }

        /// Exactly as `transfer`, except the origin must be root and the source
        /// account may be specified.
        ///
        /// The dispatch origin for this call must be _Root_.
        ///
        /// - `source`: The sender of the transfer.
        /// - `dest`: The recipient of the transfer.
        /// - `currency_id`: currency type.
        /// - `amount`: free balance amount to tranfer.
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::force_transfer())]
        pub fn force_transfer(
            origin: OriginFor<T>,
            source: <<T as Config>::Lookup as StaticLookup>::Source,
            dest: <<T as Config>::Lookup as StaticLookup>::Source,
            currency_id: T::CurrencyId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let from = <T as Config>::Lookup::lookup(source)?;
            let to = <T as Config>::Lookup::lookup(dest)?;
            Self::do_transfer(currency_id, &from, &to, amount)
        }

        /// Set the balances of a given account.
        ///
        /// This will alter `FreeBalance` and `ReservedBalance` in storage. it
        /// will also decrease the total issuance of the system
        /// (`TotalIssuance`). If the new free or reserved balance is below the
        /// existential deposit, it will reap the `AccountInfo`.
        ///
        /// The dispatch origin for this call is `root`.
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::set_balance())]
        pub fn set_balance(
            origin: OriginFor<T>,
            who: <<T as Config>::Lookup as StaticLookup>::Source,
            currency_id: T::CurrencyId,
            #[pallet::compact] new_balance: T::Balance,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let who = <T as Config>::Lookup::lookup(who)?;

            Self::try_mutate_account(&who, currency_id, |account, _| -> DispatchResult {
                let old_balance = account.clone();

                *account = new_balance;

                if new_balance > old_balance {
                    TotalIssuance::<T>::try_mutate(currency_id, |t| -> DispatchResult {
                        *t = t
                            .checked_add(&(new_balance.defensive_saturating_sub(old_balance)))
                            .ok_or(ArithmeticError::Overflow)?;
                        Ok(())
                    })?;
                } else if new_balance < old_balance {
                    TotalIssuance::<T>::try_mutate(currency_id, |t| -> DispatchResult {
                        *t = t
                            .checked_sub(&(old_balance.defensive_saturating_sub(new_balance)))
                            .ok_or(ArithmeticError::Underflow)?;
                        Ok(())
                    })?;
                }

                Self::deposit_event(Event::BalanceSet {
                    currency_id,
                    who: who.clone(),
                    balance: new_balance,
                });
                Ok(())
            })?;

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub(crate) fn deposit_consequence(
        currency_id: T::CurrencyId,
        amount: T::Balance,
        balance: &T::Balance,
    ) -> DepositConsequence {
        if amount.is_zero() {
            return DepositConsequence::Success;
        }

        if TotalIssuance::<T>::get(currency_id)
            .checked_add(&amount)
            .is_none()
        {
            return DepositConsequence::Overflow;
        }

        match balance.checked_add(&amount) {
            Some(x) => x,
            None => return DepositConsequence::Overflow,
        };

        DepositConsequence::Success
    }

    pub(crate) fn withdraw_consequence(
        currency_id: T::CurrencyId,
        amount: T::Balance,
        balance: &T::Balance,
    ) -> WithdrawConsequence<T::Balance> {
        if amount.is_zero() {
            return WithdrawConsequence::Success;
        }

        if TotalIssuance::<T>::get(currency_id)
            .checked_sub(&amount)
            .is_none()
        {
            return WithdrawConsequence::Underflow;
        }

        match balance.checked_sub(&amount) {
            Some(x) => x,
            None => return WithdrawConsequence::BalanceLow,
        };

        WithdrawConsequence::Success
    }

    // Ensure that an account can withdraw from their free balance given any
    // existing withdrawal restrictions like locks and vesting balance.
    // Is a no-op if amount to be withdrawn is zero.
    pub(crate) fn ensure_can_withdraw(
        currency_id: T::CurrencyId,
        who: &<T as module::Config>::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        if amount.is_zero() {
            return Ok(());
        }

        Self::free_balance(currency_id, who)
            .checked_sub(&amount)
            .ok_or(Error::<T>::BalanceTooLow)?;

        Ok(())
    }

    pub(crate) fn try_mutate_account<R, E>(
        who: &<T as module::Config>::AccountId,
        currency_id: T::CurrencyId,
        f: impl FnOnce(&mut T::Balance, bool) -> sp_std::result::Result<R, E>,
    ) -> sp_std::result::Result<R, E> {
        Accounts::<T>::try_mutate_exists(who, currency_id, |maybe_account| {
            let existed = maybe_account.is_some();
            let mut balance = maybe_account.take().unwrap_or_default();
            f(&mut balance, existed).map(move |result| {
                let maybe_endowed = if !existed { Some(balance) } else { None };
                *maybe_account = Some(balance);

                (maybe_endowed, existed, maybe_account.is_some(), result)
            })
        })
        .map(|(maybe_endowed, existed, exists, result)| {
            if existed && !exists {
                AccountsByCurrency::<T>::remove(currency_id, who.clone());
            } else if !existed && exists {
                AccountsByCurrency::<T>::insert(currency_id, who.clone(), ());
            }

            if let Some(endowed) = maybe_endowed {
                Self::deposit_event(Event::Endowed {
                    currency_id,
                    who: who.clone(),
                    amount: endowed,
                });
            }

            result
        })
    }

    /// Transfer some free balance from `from` to `to`. Ensure from_account
    /// allow death or new balance will not be reaped, and ensure
    /// to_account will not be removed dust.
    ///
    /// Is a no-op if value to be transferred is zero or the `from` is the same
    /// as `to`.
    pub(crate) fn do_transfer(
        currency_id: T::CurrencyId,
        from: &<T as module::Config>::AccountId,
        to: &<T as module::Config>::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        if amount.is_zero() || from == to {
            return Ok(());
        }

        if Frozen::<T>::get(currency_id) {
            return Err(sp_runtime::DispatchError::Token(
                sp_runtime::TokenError::Frozen,
            ));
        }

        Self::try_mutate_account(to, currency_id, |to_account, _existed| -> DispatchResult {
            Self::try_mutate_account(
                from,
                currency_id,
                |from_account, _existed| -> DispatchResult {
                    *from_account = from_account
                        .checked_sub(&amount)
                        .ok_or(Error::<T>::BalanceTooLow)?;
                    *to_account = to_account
                        .checked_add(&amount)
                        .ok_or(ArithmeticError::Overflow)?;

                    Self::ensure_can_withdraw(currency_id, from, amount)?;

                    Ok(())
                },
            )?;
            Ok(())
        })?;

        Self::deposit_event(Event::Transfer {
            currency_id,
            from: from.clone(),
            to: to.clone(),
            amount,
        });
        Ok(())
    }

    /// Withdraw some free balance from an account, respecting existence
    /// requirements.
    ///
    /// `change_total_issuance`:
    /// - true, decrease the total issuance by burned amount.
    /// - false, do not update the total issuance.
    ///
    /// Is a no-op if value to be withdrawn is zero.
    pub(crate) fn do_withdraw(
        currency_id: T::CurrencyId,
        who: &<T as module::Config>::AccountId,
        amount: T::Balance,
        change_total_issuance: bool,
    ) -> DispatchResult {
        if amount.is_zero() {
            return Ok(());
        }

        Self::try_mutate_account(who, currency_id, |account, _existed| -> DispatchResult {
            Self::ensure_can_withdraw(currency_id, who, amount)?;

            *account = account.defensive_saturating_sub(amount);

            if change_total_issuance {
                TotalIssuance::<T>::mutate(currency_id, |v| {
                    *v = v.defensive_saturating_sub(amount)
                });
            }

            Self::deposit_event(Event::Withdrawn {
                currency_id,
                who: who.clone(),
                amount,
            });
            Ok(())
        })?;

        Ok(())
    }

    /// Deposit some `value` into the free balance of `who`.
    ///
    /// `require_existed`:
    /// - true, the account must already exist, do not require ED.
    /// - false, possibly creating a new account, require ED if the account does
    ///   not yet exist, but except this account is in the dust removal
    ///   whitelist.
    ///
    /// `change_total_issuance`:
    /// - true, increase the issued amount to total issuance.
    /// - false, do not update the total issuance.
    pub(crate) fn do_deposit(
        currency_id: T::CurrencyId,
        who: &<T as module::Config>::AccountId,
        amount: T::Balance,
        require_existed: bool,
        change_total_issuance: bool,
    ) -> Result<T::Balance, DispatchError> {
        if amount.is_zero() {
            return Ok(amount);
        }

        Self::try_mutate_account(who, currency_id, |account, existed| -> DispatchResult {
            if require_existed {
                ensure!(existed, Error::<T>::DeadAccount);
            }

            let new_total_issuance = Self::total_issuance(currency_id)
                .checked_add(&amount)
                .ok_or(ArithmeticError::Overflow)?;
            if change_total_issuance {
                TotalIssuance::<T>::mutate(currency_id, |v| *v = new_total_issuance);
            }
            *account = account.defensive_saturating_add(amount);
            Ok(())
        })?;

        Self::deposit_event(Event::Deposited {
            currency_id,
            who: who.clone(),
            amount,
        });
        Ok(amount)
    }

    pub(crate) fn free_balance(
        currency_id: T::CurrencyId,
        who: &<T as module::Config>::AccountId,
    ) -> T::Balance {
        Self::accounts(who, currency_id)
    }

    pub fn freeze_currency(currency_id: T::CurrencyId) {
        Frozen::<T>::insert(currency_id, true)
    }

    pub fn unfreeze_currency(currency_id: T::CurrencyId) {
        Frozen::<T>::insert(currency_id, false)
    }
}

impl<T: Config> fungibles::Inspect<<T as module::Config>::AccountId> for Pallet<T> {
    type AssetId = T::CurrencyId;
    type Balance = T::Balance;

    fn total_issuance(asset_id: Self::AssetId) -> Self::Balance {
        Self::total_issuance(asset_id)
    }

    fn minimum_balance(_asset_id: Self::AssetId) -> Self::Balance {
        Self::Balance::zero()
    }

    fn balance(asset_id: Self::AssetId, who: &<T as module::Config>::AccountId) -> Self::Balance {
        Self::accounts(who, asset_id)
    }

    fn total_balance(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
    ) -> Self::Balance {
        Self::accounts(who, asset_id)
    }

    fn reducible_balance(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        _preservation: Preservation,
        _force: Fortitude,
    ) -> Self::Balance {
        Self::accounts(who, asset_id)
    }

    fn can_deposit(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        amount: Self::Balance,
        _provenance: Provenance,
    ) -> DepositConsequence {
        Self::deposit_consequence(asset_id, amount, &Self::accounts(who, asset_id))
    }

    fn can_withdraw(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        Self::withdraw_consequence(asset_id, amount, &Self::accounts(who, asset_id))
    }

    fn asset_exists(asset: Self::AssetId) -> bool {
        TotalIssuance::<T>::contains_key(asset)
    }
}

impl<T: Config> fungibles::InspectFreeze<<T as module::Config>::AccountId> for Pallet<T> {
    type Id = ();

    fn balance_frozen(
        asset: Self::AssetId,
        _id: &Self::Id,
        _who: &<T as module::Config>::AccountId,
    ) -> Self::Balance {
        if Frozen::<T>::get(asset) {
            TotalIssuance::<T>::get(asset)
        } else {
            Zero::zero()
        }
    }

    fn can_freeze(
        asset: Self::AssetId,
        _id: &Self::Id,
        _who: &<T as module::Config>::AccountId,
    ) -> bool {
        Frozen::<T>::get(asset)
    }
}

impl<T: Config> fungibles::MutateFreeze<<T as module::Config>::AccountId> for Pallet<T> {
    fn set_freeze(
        asset: Self::AssetId,
        _id: &Self::Id,
        _who: &<T as module::Config>::AccountId,
        _amount: Self::Balance,
    ) -> DispatchResult {
        Self::freeze_currency(asset);

        Ok(())
    }

    fn extend_freeze(
        _asset: Self::AssetId,
        _id: &Self::Id,
        _who: &<T as module::Config>::AccountId,
        _amount: Self::Balance,
    ) -> DispatchResult {
        Ok(())
    }

    fn thaw(
        asset: Self::AssetId,
        _id: &Self::Id,
        _who: &<T as module::Config>::AccountId,
    ) -> DispatchResult {
        Self::unfreeze_currency(asset);

        Ok(())
    }
}

impl<T: Config> fungibles::Mutate<<T as module::Config>::AccountId> for Pallet<T> {
    fn mint_into(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        Self::deposit_consequence(asset_id, amount, &Self::accounts(who, asset_id))
            .into_result()?;
        // do not require existing
        Self::do_deposit(asset_id, who, amount, false, true)
    }

    fn burn_from(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        amount: Self::Balance,
        // TODO: Respect precision
        _precision: Precision,
        // TODO: Respect fortitude
        _fortitude: Fortitude,
    ) -> Result<Self::Balance, DispatchError> {
        let extra = Self::withdraw_consequence(asset_id, amount, &Self::accounts(who, asset_id))
            .into_result(false)?;
        let actual = amount.defensive_saturating_add(extra);
        // allow death
        Self::do_withdraw(asset_id, who, actual, true).map(|_| actual)
    }

    fn transfer(
        asset_id: Self::AssetId,
        source: &<T as module::Config>::AccountId,
        dest: &<T as module::Config>::AccountId,
        amount: T::Balance,
        _preservation: Preservation,
    ) -> Result<T::Balance, DispatchError> {
        Self::do_transfer(asset_id, source, dest, amount).map(|_| amount)
    }
}

impl<T: Config> fungibles::Unbalanced<<T as module::Config>::AccountId> for Pallet<T> {
    fn handle_dust(_dust: fungibles::Dust<<T as module::Config>::AccountId, Self>) {
        // Dust is handled in account mutate method
    }

    fn write_balance(
        asset_id: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        amount: Self::Balance,
    ) -> Result<Option<Self::Balance>, DispatchError> {
        let max_reduction = <Self as fungibles::Inspect<_>>::reducible_balance(
            asset_id,
            who,
            Preservation::Expendable,
            Fortitude::Force,
        );

        // Balance is the same type and will not overflow
        Self::try_mutate_account(who, asset_id, |account, _| -> Result<(), DispatchError> {
            // Make sure the reduction (if there is one) is no more than the maximum
            // allowed.
            let reduction = account.saturating_sub(amount);
            ensure!(reduction <= max_reduction, Error::<T>::BalanceTooLow);

            *account = amount;
            Self::deposit_event(Event::BalanceSet {
                currency_id: asset_id,
                who: who.clone(),
                balance: *account,
            });

            Ok(())
        })?;

        Ok(None)
    }

    fn set_total_issuance(asset_id: Self::AssetId, amount: Self::Balance) {
        // Balance is the same type and will not overflow
        TotalIssuance::<T>::mutate(asset_id, |t| *t = amount);

        Self::deposit_event(Event::TotalIssuanceSet {
            currency_id: asset_id,
            amount,
        });
    }

    fn decrease_balance(
        asset: Self::AssetId,
        who: &<T as module::Config>::AccountId,
        mut amount: Self::Balance,
        precision: Precision,
        preservation: Preservation,
        force: Fortitude,
    ) -> Result<Self::Balance, DispatchError> {
        let old_balance =
            <Pallet<T> as fungibles::Inspect<<T as module::Config>::AccountId>>::balance(
                asset, who,
            );
        let free =
            <Pallet<T> as fungibles::Inspect<<T as module::Config>::AccountId>>::reducible_balance(
                asset,
                who,
                preservation,
                force,
            );
        if let Precision::BestEffort = precision {
            amount = amount.min(free);
        }
        let new_balance = old_balance
            .checked_sub(&amount)
            .ok_or(TokenError::FundsUnavailable)?;
        let _dust_amount = Self::write_balance(asset, who, new_balance)?.unwrap_or_default();

        // here just return decrease amount, shouldn't count the dust_amount
        Ok(old_balance.saturating_sub(new_balance))
    }
}
