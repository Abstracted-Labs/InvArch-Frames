//! Benchmarks for IPF Pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::{Config, RawOrigin};

benchmarks! {
    where_clause {
        where T: pallet::Config
    }
    // Mint IPF
    mint {
        let caller: T::AccountId = whitelisted_caller();
        let metadata: Vec<u8> = vec![1];
    }: _(RawOrigin::Signed(caller), metadata, Default::default())

    // Burn IPF
    burn {
        let caller: T::AccountId = whitelisted_caller();
        let ipf_id: u32 = 0;
    }: _(RawOrigin::Signed(caller), T::IpfId::from(ipf_id))
}

impl_benchmark_test_suite!(Ipf, crate::mock::new_test_ext(), crate::mock::Test);
