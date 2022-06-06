//! Benchmarks for IPT Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{
    account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller, Box,
};
use frame_system::RawOrigin;
use primitives::{InvArchLicenses, OneOrPercent::*, SubIptInfo};
use sp_io::hashing::blake2_256;
use sp_runtime::{traits::UniqueSaturatedInto, Percent};

pub type Balance = u128;
pub type ExistentialDeposit = u128;

macro_rules! percent {
    ($x:expr) => {
        ZeroPoint(Percent::from_percent($x))
    };
}

const SEED: u32 = 0;

fn dollar(d: u32) -> Balance {
    let d: Balance = d.into();
    d.saturating_mul(1_000_000_000_000_000_000)
}

benchmarks! {
    where_clause {
        where T: pallet::Config<Licenses = InvArchLicenses>
    }

    mint {
        let caller: T::AccountId = whitelisted_caller();
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![(caller.clone(), amount)];

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(caller.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

    }: _(RawOrigin::Signed(caller), (T::IptId::from(0u32), None), amount, target)

    burn {
        let caller: T::AccountId = whitelisted_caller();
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![(caller.clone(), amount)];

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(caller.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

        Pallet::<T>::internal_mint((T::IptId::from(0u32), None), target.clone(), amount.clone())?;

        Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), (T::IptId::from(0u32), None), amount, target.clone())?;

    }: _(RawOrigin::Signed(caller), (T::IptId::from(0u32), None), amount, target)

    operate_multisig {
        let alice: T::AccountId = account("Alice", 0, SEED);
        let bob: T::AccountId = account("Bob", 0, SEED);
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let call: <T as pallet::Config>::Call = frame_system::Call::<T>::remark {
            remark: vec![0; 0 as usize],
        }.into();
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![
            (alice.clone(), amount),
            (bob.clone(), amount)
            ];

        <T as pallet::Config>::Currency::make_free_balance_be(&alice, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(alice.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

        Pallet::<T>::internal_mint((T::IptId::from(0u32), None), target.clone(), amount.clone())?;

        Pallet::<T>::mint(RawOrigin::Signed(alice.clone()).into(), (T::IptId::from(0u32), None), amount, target.clone())?;

    }: _(RawOrigin::Signed(alice), false, (T::IptId::from(0u32), None), Box::new(call))

    vote_multisig {
        let alice: T::AccountId = account("Alice", 0, SEED);
        let bob: T::AccountId = account("Bob", 0, SEED);
        let vader: T::AccountId = account("Vader", 0, SEED);
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let call: <T as pallet::Config>::Call = frame_system::Call::<T>::remark {
            remark: vec![0; 0 as usize],
        }.into();
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![
            (alice.clone(), amount),
            (bob.clone(), amount),
            (vader.clone(), amount)
            ];

        <T as pallet::Config>::Currency::make_free_balance_be(&alice, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(alice.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

        Pallet::<T>::internal_mint((T::IptId::from(0u32), None), target.clone(), amount.clone())?;

        Pallet::<T>::mint(RawOrigin::Signed(alice.clone()).into(), (T::IptId::from(0u32), None), amount, target.clone())?;

        Pallet::<T>::operate_multisig(RawOrigin::Signed(alice.clone()).into(), false, (T::IptId::from(0u32), None), Box::new(call.clone()))?;

    }: _(RawOrigin::Signed(alice), (T::IptId::from(0u32), None), blake2_256(&call.encode()))

    withdraw_vote_multisig {
        let alice: T::AccountId = account("Alice", 0, SEED);
        let bob: T::AccountId = account("Bob", 0, SEED);
        let vader: T::AccountId = account("Vader", 0, SEED);
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let call: <T as pallet::Config>::Call = frame_system::Call::<T>::remark {
            remark: vec![0; 0 as usize],
        }.into();
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![
            (alice.clone(), amount),
            (bob.clone(), amount),
            (vader.clone(), amount)
            ];

        <T as pallet::Config>::Currency::make_free_balance_be(&alice, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(alice.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

        Pallet::<T>::internal_mint((T::IptId::from(0u32), None), target.clone(), amount.clone())?;

        Pallet::<T>::mint(RawOrigin::Signed(alice.clone()).into(), (T::IptId::from(0u32), None), amount, target.clone())?;

        Pallet::<T>::operate_multisig(RawOrigin::Signed(alice.clone()).into(), false, (T::IptId::from(0u32), None), Box::new(call.clone()))?;

        Pallet::<T>::vote_multisig(RawOrigin::Signed(alice.clone()).into(), (T::IptId::from(0u32), None), blake2_256(&call.encode()))?;

    }: _(RawOrigin::Signed(alice), (T::IptId::from(0u32), None), blake2_256(&call.encode()))

    create_sub_asset {
        let alice: T::AccountId = account("Alice", 0, SEED);
        let bob: T::AccountId = account("Bob", 0, SEED);
        let vader: T::AccountId = account("Vader", 0, SEED);
        let amount: <T as pallet::Config>::Balance = 300u32.into();
        let target: T::AccountId = account("target", 0, SEED);
        let sub_assets: SubAssetsWithEndowment<T> = vec![(
            SubIptInfo {id: T::IptId::from(0u32), metadata: Default::default()}, (account("target", 0, SEED), 500u32.into()) 
        )];
        let base_currency_amount = dollar(1000);
        let endowed_accounts = vec![
            (alice.clone(), amount),
            (bob.clone(), amount),
            (vader.clone(), amount)
        ];

        <T as pallet::Config>::Currency::make_free_balance_be(&alice, base_currency_amount.unique_saturated_into());

        Pallet::<T>::create(alice.clone(), T::IptId::from(0u32), endowed_accounts, Default::default(), InvArchLicenses::GPLv3, percent!(50), One, false);

        Pallet::<T>::internal_mint((T::IptId::from(0u32), None), target.clone(), amount.clone())?;

        Pallet::<T>::mint(RawOrigin::Signed(alice.clone()).into(), (T::IptId::from(0u32), None), amount, target.clone())?;



    }: _(RawOrigin::Signed(alice), T::IptId::from(0u32), sub_assets)
}

impl_benchmark_test_suite!(Ipt, crate::mock::new_test_ext(), crate::mock::Test,);
