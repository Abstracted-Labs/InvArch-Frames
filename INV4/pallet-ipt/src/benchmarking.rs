//! Benchmarks for IPT Pallet
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

  operate_multisig {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  vote_multisig {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  withdraw_vote_multisig {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}

  create_sub_asset {
      let s in 0 .. 100;
      let caller: T::AccountId = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), s)
  verify {}
}

impl_benchmark_test_suite!(Ipt, crate::mock::new_test_ext(), crate::mock::Test,);
