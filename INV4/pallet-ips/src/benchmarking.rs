//! Benchmarks for IPS Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use ipl::LicenseList;
use primitives::OneOrPercent::{One, ZeroPoint};
use sp_runtime::Percent;

type Hash = sp_core::H256;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Eq, PartialEq)]
pub enum InvArchLicenses {
    Apache2,
    GPLv3,
    Custom(Vec<u8>, Hash),
}

impl LicenseList for InvArchLicenses {
    type IpfsHash = Hash; // License IPFS hash.
    type LicenseMetadata = Vec<u8>; // License name.

    fn get_hash_and_metadata(&self) -> (Self::LicenseMetadata, Self::IpfsHash) {
        match self {
            InvArchLicenses::Apache2 => (
                vec![65, 112, 97, 99, 104, 97, 32, 118, 50, 46, 48],
                [
                    7, 57, 92, 251, 234, 183, 217, 144, 220, 196, 201, 132, 176, 249, 18, 224, 237,
                    201, 2, 113, 146, 78, 111, 152, 92, 71, 16, 228, 87, 39, 81, 142,
                ]
                .into(),
            ),
            InvArchLicenses::GPLv3 => (
                vec![71, 78, 85, 32, 71, 80, 76, 32, 118, 51],
                [
                    72, 7, 169, 24, 30, 7, 200, 69, 232, 27, 10, 138, 130, 253, 91, 158, 210, 95,
                    127, 37, 85, 41, 106, 136, 66, 116, 64, 35, 252, 195, 69, 253,
                ]
                .into(),
            ),
            InvArchLicenses::Custom(metadata, hash) => (metadata.clone(), *hash),
        }
    }
}

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
    }: _(RawOrigin::Signed(caller), metadata, data, true, None, license.into(), percent!(50), One, false)

    destroy {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s))

    append {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), T::IpsId::from(0u32), Default::default(), Some(vec![s.try_into().unwrap()]))

    remove {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
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
    }: _(RawOrigin::Signed(caller), T::IpsId::from(s), license, percent!(50), One, false)
}

impl_benchmark_test_suite!(Ips, crate::mock::new_test_ext(), crate::mock::Test,);
