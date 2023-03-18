#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{pallet_prelude::*, traits::Get};
use pallet_inv4::origin::{INV4Origin, MultisigInternalOrigin};
use sp_std::{ops::Div, prelude::*, vec};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    where_clause {
      where
        Result<
                INV4Origin<
                        T,
                    <T as pallet_inv4::Config>::CoreId,
                    <T as frame_system::Config>::AccountId,
                    >,
            <T as frame_system::Config>::RuntimeOrigin,
            >: From<<T as frame_system::Config>::RuntimeOrigin>,

    <T as pallet_inv4::Config>::CoreId: Into<u32>,

    [u8; 32]: From<<T as frame_system::Config>::AccountId>,

    <T as frame_system::Config>::RuntimeOrigin:
    From<INV4Origin<T, <T as pallet_inv4::Config>::CoreId, <T as frame_system::Config>::AccountId>>,
}

    send_call {
        let c in 0 .. T::MaxWeightedLength::get();

        let call = vec![u8::MAX; c as usize];
        let destination = T::Chains::benchmark_mock();
        let weight = Weight::from_ref_time(100_000_000);

    }: _(INV4Origin::Multisig(MultisigInternalOrigin::new(0u32.into())), destination.clone(), weight, call.clone())
        verify {
            assert_last_event::<T>(Event::CallSent {
                sender: 0u32.into(),
                destination,
                call,
            }.into());
        }

    transfer_assets {
        let asset: <<T as Config>::Chains as ChainList>::ChainAssets = T::Chains::benchmark_mock().get_main_asset();
        let amount: u128 = u128::MAX.div(4u128);
        let to: T::AccountId = whitelisted_caller();

    }: _(INV4Origin::Multisig(MultisigInternalOrigin::new(0u32.into())), asset.clone(), amount, to.clone())
        verify {
            assert_last_event::<T>(Event::AssetsTransferred {
                chain: asset.clone().get_chain(),
                asset,
                amount,
                from: 0u32.into(),
                to,
            }.into());
        }
}