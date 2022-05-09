//! Benchmarks for IPF Pallet
#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
  mint {
    let s in 0 .. 100;
    let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  burn {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  send {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}
}

impl_benchmark_test_suite!(Ipf, crate::mock::new_test_ext(), crate::mock::Test,);
