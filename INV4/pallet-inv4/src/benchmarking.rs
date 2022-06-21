#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::OneOrPercent::{One, ZeroPoint};
// use sp_core::H256;
use sp_runtime::{traits::UniqueSaturatedInto, Percent};

// const MOCK_DATA: [u8; 32] = [
//     12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
//     0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
// ];

// pub const MOCK_DATA_SECONDARY: [u8; 32] = [
//     47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218, 0,
//     230, 247, 32, 73, 152, 66, 243, 27, 92, 95, 12,
// ];

pub const MOCK_METADATA: &'static [u8] = &[
    12, 47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218,
    0, 230, 247, 32, 73, 152, 66, 243, 27, 92, 95,
];

// pub const MOCK_METADATA_SECONDARY: &'static [u8] = &[
//     47, 182, 72, 140, 51, 139, 219, 171, 74, 247, 18, 123, 28, 200, 236, 221, 85, 25, 12, 218, 0,
//     230, 247, 32, 73, 152, 66, 243, 27, 92, 95, 12,
// ];

// type Hash = sp_core::H256;

// pub trait LicenseList {
//     type IpfsHash: core::hash::Hash;
//     type LicenseMetadata;

//     fn get_hash_and_metadata(&self) -> (Self::LicenseMetadata, Self::IpfsHash);
// }

// #[derive(Debug, Clone, Encode, Decode, TypeInfo, Eq, PartialEq)]
// pub enum InvArchLicenses {
//     Apache2,
//     GPLv3,
//     Custom(Vec<u8>, Hash),
// }

// impl LicenseList for InvArchLicenses {
//     type IpfsHash = Hash; // License IPFS hash.
//     type LicenseMetadata = Vec<u8>; // License name.

//     fn get_hash_and_metadata(&self) -> (Self::LicenseMetadata, Self::IpfsHash) {
//         match self {
//             InvArchLicenses::Apache2 => (
//                 vec![65, 112, 97, 99, 104, 97, 32, 118, 50, 46, 48],
//                 [
//                     7, 57, 92, 251, 234, 183, 217, 144, 220, 196, 201, 132, 176, 249, 18, 224, 237,
//                     201, 2, 113, 146, 78, 111, 152, 92, 71, 16, 228, 87, 39, 81, 142,
//                 ]
//                 .into(),
//             ),
//             InvArchLicenses::GPLv3 => (
//                 vec![71, 78, 85, 32, 71, 80, 76, 32, 118, 51],
//                 [
//                     72, 7, 169, 24, 30, 7, 200, 69, 232, 27, 10, 138, 130, 253, 91, 158, 210, 95,
//                     127, 37, 85, 41, 106, 136, 66, 116, 64, 35, 252, 195, 69, 253,
//                 ]
//                 .into(),
//             ),
//             InvArchLicenses::Custom(metadata, hash) => (metadata.clone(), *hash),
//         }
//     }
// }

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

// pub enum InvArchLicenses {
//   Apache2,
//   GPLv3,
//   Custom,
// }

benchmarks! {
  create_ips {
    let bob: T::AccountId = account("Bob", 0, SEED);
    let metadata_1: Vec<u8> = MOCK_METADATA.to_vec();
    let assets = vec![AnyIdOf::<T>::from(pallet::AnyId::RmrkCollection(0u32))];
    let ipl_license = ipl::LicenseList; // TODO: rewrite
    let base_currency_amount = dollar(1000);

    <T as pallet::Config>::Currency::make_free_balance_be(&bob, base_currency_amount.unique_saturated_into());

    // TODO: inner create ips

  }: _(RawOrigin::Signed(bob), metadata_1, assets, true, ipl_license, percent!(50), One, false)
}

impl_benchmark_test_suite!(Inv4, crate::mock::new_test_ext(), crate::mock::Test,);