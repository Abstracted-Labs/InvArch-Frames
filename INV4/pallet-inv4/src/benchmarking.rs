#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::OneOrPercent::{One, ZeroPoint};
use sp_core::H256;
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

type Hash = sp_core::H256;

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
  create_ips {
    let bob: T::AccountId = account("Bob", 0, SEED);
    let metadata_1: Vec<u8> = MOCK_METADATA.to_vec();
    let assets = vec![AnyIdOf::<T>::from(pallet::AnyId::RmrkCollection(0u32))];
    let ipl_license = ipl::LicenseList::GPLv3; // TODO: rewrite
    let base_currency_amount = dollar(1000);

    <T as pallet::Config>::Currency::make_free_balance_be(&bob, base_currency_amount.unique_saturated_into());

    // TODO: inner create ips

  }: _(RawOrigin::Signed(bob), metadata_1, assets, true, ipl_license, percent!(50), One, false)
}

impl_benchmark_test_suite!(Inv4, crate::mock::new_test_ext(), crate::mock::Test,);
