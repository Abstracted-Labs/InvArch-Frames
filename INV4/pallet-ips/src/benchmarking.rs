//! Benchmarks for IPS Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::{
    InvArchLicenses,
    OneOrPercent::{One, ZeroPoint},
};
use sp_runtime::Percent;

macro_rules! percent {
    ($x:expr) => {
        ZeroPoint(Percent::from_percent($x))
    };
}

benchmarks! {
    where_clause {
        where T: ipl::Config<Licenses = InvArchLicenses>
    }
    create_ips {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let license = InvArchLicenses::GPLv3;

        //crate::Ipf::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata, data)?;
    }: _(RawOrigin::Signed(caller), metadata, data, true, None, license, percent!(50), One, false)

    destroy {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let license = InvArchLicenses::GPLv3;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s))

    append {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
        let data: Vec<<T as ipf::Config>::IpfId> = vec![];
        let license = InvArchLicenses::GPLv3;

        //crate::Ipf::<T>::mint(RawOrigin::Signed(caller.clone()).into(), metadata, data)?;

        Pallet::<T>::create_ips(RawOrigin::Signed(caller.clone()).into(), metadata, data, true, None, license, percent!(50), One, false)?;

    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    remove {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();

        Pallet::<T>::append(RawOrigin::Signed(caller.clone()).into(), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))?;
    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    allow_replica {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s))

    disallow_replica {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s))

    create_replica {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
        let license = InvArchLicenses::GPLv3;

        Pallet::<T>::allow_replica(RawOrigin::Signed(caller.clone()).into(), T::IpsId::from(s))?;
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s), license, percent!(50), One, false)
}

impl_benchmark_test_suite!(Ips, crate::mock::new_test_ext(), crate::mock::Test,);
