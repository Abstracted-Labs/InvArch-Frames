//! Benchmarks for IPF Pallet
#![cfg(feature = "runtime-benchmarks")]

use crate::IpfInfo;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{RawOrigin, Config, Pallet};
use sp_runtime::traits::StaticLookup;

const SEED: u32 = 0;

benchmarks! {
  // Mint IPF
  mint {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1],
        let data = H256
    }: _(RawOrigin::Signed(caller), metadata, data)

//   // Burn IPF
//   burn {
//     let caller: T::AccountId = whitelisted_caller();
//     let ipf_id: u64 = 0,
// }: _(RawOrigin::Signed(caller), ipf_id)

//   // Send IPF
//   send {
//         let caller: T::AccountId = whitelisted_caller();
//         let ipf_id: u64 = 0,
//         let target = T::Lookup::unlookup(to);;
//     }: _(RawOrigin::Signed(caller), ipf_id, target)
}

impl_benchmark_test_suite!(Ipf, crate::mock::new_test_ext(), crate::mock::Test,);
