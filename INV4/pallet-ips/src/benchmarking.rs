//! Benchmarks for IPS Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::{
    InvArchLicenses, IpsInfo, IpsType,
    OneOrPercent::{One, ZeroPoint},
    Parentage,
};
use sp_core::H256;
use sp_runtime::{traits::UniqueSaturatedInto, Percent};

const MOCK_DATA: [u8; 32] = [
    12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
    0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
];

pub const MOCK_DATA_SECONDARY: [u8; 32] = [
    47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218, 0,
    230, 247, 32, 73, 152, 66, 243, 27, 92, 95, 12,
];

pub const MOCK_METADATA: &'static [u8] = &[
    12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
    0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
];

pub const MOCK_METADATA_SECONDARY: &'static [u8] = &[
    47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218, 0,
    230, 247, 32, 73, 152, 66, 243, 27, 92, 95, 12,
];

const SEED: u32 = 0;

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
        let bob: T::AccountId = account("Bob", 0, SEED);
        let alice: T::AccountId = account("Alice", 0, SEED);
        let metadata_1: Vec<u8> = MOCK_METADATA.to_vec();
        let metadata_2: Vec<u8> = MOCK_METADATA_SECONDARY.to_vec();
        let data = vec![T::IpfId::from(0u32)];
        let ipf_data_1 = H256::from(MOCK_DATA);
        let ipf_data_2 = H256::from(MOCK_DATA_SECONDARY);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);

        <T as pallet::Config>::Currency::make_free_balance_be(&bob, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(bob.clone()).into(), metadata_1.clone(), ipf_data_1)?;

        ipf::Pallet::<T>::mint(RawOrigin::Signed(alice).into(), metadata_2, ipf_data_2)?;

    }: _(RawOrigin::Signed(bob), metadata_1, data, true, None, license, percent!(50), One, false)

    destroy {
        let bob: T::AccountId = account("Bob", 0, SEED);
        let metadata: Vec<u8> = MOCK_METADATA.to_vec();
        let data = vec![T::IpfId::from(0u32)];
        let ipf_data = H256::from(MOCK_DATA);
        let license = InvArchLicenses::GPLv3;
        let base_currency_amount = dollar(1000);
        let ips_id = T::IpsId::from(0u32);
        let ips_account = primitives::utils::multi_account_id::<T, <T as Config>::IpsId>(
            ips_id, None,
        );

        <T as pallet::Config>::Currency::make_free_balance_be(&bob, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(bob.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(bob.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

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

    }: _(RawOrigin::Signed(ips_account), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    allow_replica {
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
        // let ips_info = IpsInfo::<T>::from(
        //     Parentage::Parent(ips_account),
        //     metadata,
        //     data,
        //     IpsType::Normal,
        //     true,
        // );

        <T as pallet::Config>::Currency::make_free_balance_be(&caller, base_currency_amount.unique_saturated_into());

        ipf::Pallet::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata.clone(), ipf_data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, true)?;

        // TODO: change value

        // Pallet::<T>::allow_replica(RawOrigin::Signed(ips_account.clone()).into(), T::IpsId::from(ips_info))?;


    }: _(RawOrigin::Signed(ips_account), T::IpsId::from(ips_id))

    disallow_replica {
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

    create_replica {
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

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license.clone(), percent!(50), One, false)?;

        Pallet::<T>::allow_replica(RawOrigin::Signed(ips_account.clone()).into(), T::IpsId::from(0u32))?;

        // TODO: change value

    }: _(RawOrigin::Signed(ips_account), T::IpsId::from(ips_id), license, percent!(50), One, true)
}

impl_benchmark_test_suite!(Ips, crate::mock::new_test_ext(), crate::mock::Test,);
