//! Benchmarks for IPL Pallet
#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
  set_permission {
    let s in 0 .. 100;
    let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s, _, _, _)
  verify {}

  set_asset_weight {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s, _, _)
  verify {}
}

impl_benchmark_test_suite!(Ipl, crate::mock::new_test_ext(), crate::mock::Test,);
