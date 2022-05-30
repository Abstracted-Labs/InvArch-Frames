//! Benchmarks for IPS Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::{ 
    InvArchLicenses,
    OneOrPercent::{One, ZeroPoint}
};
use sp_core::H256;
use sp_runtime::{traits::UniqueSaturatedInto, Percent};

const MOCK_DATA: [u8; 32] = [
    12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
    0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
];

pub type Balance = u128;

fn dollar(d: u32) -> Balance {
    let d: Balance = d.into();
    d.saturating_mul(1_000_000_000_000_000_000)
}

macro_rules! percent {
    ($x:expr) => {
        ZeroPoint(Percent::from_percent($x))
    };
}

benchmarks! {
    where_clause {
        where T: ipl::Config<Licenses = InvArchLicenses> + frame_system::Config<Hash = H256>
    }
    create_ips {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;
    }: _(RawOrigin::Signed(caller), metadata, data, true, None, license, percent!(50), One, false)

    destroy {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);
        let ips_id = T::IpsId::from(0u32);
        let ips_account = primitives::utils::multi_account_id::<T, <T as Config>::IpsId>(
            ips_id, None,
        );

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

    }: _(RawOrigin::Signed(ips_account), T::IpsId::from(0u32))

    append {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    remove {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);
        let ips_id = T::IpsId::from(0u32);
        let ips_account = primitives::utils::multi_account_id::<T, <T as Config>::IpsId>(
            ips_id, None,
        );

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

        Pallet::<T>::append(RawOrigin::Signed(caller.clone()).into(), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))?;

        // TODO: set permision WIP

    }: _(RawOrigin::Signed(ips_account), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    allow_replica {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

        // TODO: set permision WIP

    }: _(RawOrigin::Signed(caller), T::IpsId::from(s))

    disallow_replica {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

        // TODO: set permision WIP

    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32))

    create_replica {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license.clone(), percent!(50), One, false)?;

        Pallet::<T>::allow_replica(RawOrigin::Signed(caller.clone()).into(), T::IpsId::from(0u32))?;

        // TODO: set permision WIP

    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32), license, percent!(50), One, false)
}

impl_benchmark_test_suite!(Ips, crate::mock::new_test_ext(), crate::mock::Test,);
