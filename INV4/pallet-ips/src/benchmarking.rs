//! Benchmarks for IPS Pallet
#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
  create_ips {
    let s in 0 .. 100;
    let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  destroy {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  append {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  remove {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  allow_replica {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  disallow_replica {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  create_replica {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}
}

impl_benchmark_test_suite!(Ips, crate::mock::new_test_ext(), crate::mock::Test,);
