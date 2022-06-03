//! Benchmarks for IPL Pallet
#![cfg(feature = "runtime-benchmarks")]

pub use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::{InvArchLicenses, OneOrPercent::*};
use sp_runtime::Percent;

const SEED: u32 = 0;

macro_rules! percent {
    ($x:expr) => {
        ZeroPoint(Percent::from_percent($x))
    };
}

benchmarks! {
  where_clause {
    where T: pallet::Config<Licenses = InvArchLicenses>
}
  set_permission {
      let caller: T::AccountId = account("caller", 0, SEED);
      let sub_asset: T::IplId = Default::default();
      let ipl_id = T::IplId::from(0u32);
      let ipl_account = primitives::utils::multi_account_id::<T, <T as Config>::IplId>(
          ipl_id, None,
      );

      Pallet::<T>::create(T::IplId::from(ipl_id), InvArchLicenses::GPLv3, percent!(50), One, false);

  }: _(RawOrigin::Signed(ipl_account), T::IplId::from(0u32), sub_asset, Default::default(), true)

  set_asset_weight {
      let caller: T::AccountId = account("caller", 0, SEED);
      let sub_asset: T::IplId = Default::default();
      let ipl_id = T::IplId::from(0u32);
      let ipl_account = primitives::utils::multi_account_id::<T, <T as Config>::IplId>(
          ipl_id, None,
      );

      Pallet::<T>::create(T::IplId::from(ipl_id), InvArchLicenses::GPLv3, percent!(50), One, false);

      Pallet::<T>::set_permission(RawOrigin::Signed(ipl_account.clone()).into(),T::IplId::from(0u32), sub_asset, Default::default(), true)?;
  }: _(RawOrigin::Signed(ipl_account), T::IplId::from(0u32), sub_asset, percent!(30))
}

impl_benchmark_test_suite!(Ipl, crate::mock::new_test_ext(), crate::mock::Test,);
