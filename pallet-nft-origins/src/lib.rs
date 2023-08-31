#![cfg_attr(not(feature = "std"), no_std)]

pub mod chains;
pub mod location;
pub mod origin;
pub mod xcm_converters;

pub use chains::ChainVerifier;
pub use location::{Collection, Nft, NftLocation, Parachain};
pub use origin::NftOrigin;
pub use xcm_converters::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{
        chains::ChainVerifier,
        location::{Collection, Nft, NftLocation, Parachain},
        origin::NftOrigin,
    };
    use frame_support::{
        dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::boxed::Box;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_xcm::Config {
        type Chains: ChainVerifier;

        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type RuntimeCall: Parameter
            + Dispatchable<
                RuntimeOrigin = <Self as Config>::RuntimeOrigin,
                PostInfo = PostDispatchInfo,
            > + GetDispatchInfo
            + From<frame_system::Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>;

        type RegisteredCalls: Parameter
            + Dispatchable<
                RuntimeOrigin = <Self as Config>::RuntimeOrigin,
                PostInfo = PostDispatchInfo,
            > + GetDispatchInfo;

        type RuntimeOrigin: From<Origin> + From<<Self as frame_system::Config>::RuntimeOrigin>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_registered_chain)]
    pub type RegisteredChains<T: Config> = StorageMap<_, Twox128, xcm::latest::Junction, Parachain>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::origin]
    pub type Origin = NftOrigin;

    #[pallet::error]
    pub enum Error<T> {
        SendingFailed,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        NftCalledTestFunction {
            nft_location: crate::location::NftLocation,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        Result<NftOrigin, <T as frame_system::Config>::RuntimeOrigin>:
            From<<T as frame_system::Config>::RuntimeOrigin>,
    {
        #[pallet::call_index(0)]
        #[pallet::weight({
            let dispatch_info = call.get_dispatch_info();
			      (
				        dispatch_info.weight,
				        dispatch_info.class,
			      )
		    })]
        pub fn dispatch_as_nft(
            verifier: OriginFor<T>,
            collection: Collection,
            nft: Nft,
            call: Box<<T as Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {
            let chain = crate::origin::ensure_verifier::<
                T,
                <T as frame_system::Config>::RuntimeOrigin,
            >(verifier)?;

            let nft_location = NftLocation::new(chain, collection, nft);

            (*call).dispatch(NftOrigin::Nft(nft_location).into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight({
            let dispatch_info = registered_call.get_dispatch_info();
			      (
				        dispatch_info.weight,
				        dispatch_info.class,
			      )
		    })]
        pub fn dispatch_registered_call_as_nft(
            verifier: OriginFor<T>,
            collection: Collection,
            nft: Nft,
            registered_call: Box<<T as Config>::RegisteredCalls>,
        ) -> DispatchResultWithPostInfo {
            let chain = crate::origin::ensure_verifier::<
                T,
                <T as frame_system::Config>::RuntimeOrigin,
            >(verifier)?;

            let nft_location = NftLocation::new(chain, collection, nft);

            (*registered_call).dispatch(NftOrigin::Nft(nft_location).into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(1)]
        pub fn set_registered_chain(
            _: OriginFor<T>,
            verifier: xcm::latest::Junction,
            chain: Option<Parachain>,
        ) -> DispatchResult {
            RegisteredChains::<T>::set(verifier, chain);

            Ok(())
        }

        // \/ TEST CALLS \/

        #[pallet::call_index(90)]
        #[pallet::weight(1)]
        pub fn test_nft_location(nft: OriginFor<T>) -> DispatchResult {
            let nft_location =
                crate::origin::ensure_nft::<T, <T as frame_system::Config>::RuntimeOrigin>(nft)?;

            Self::deposit_event(Event::<T>::NftCalledTestFunction { nft_location });

            Ok(())
        }

        #[pallet::call_index(91)]
        #[pallet::weight(1)]
        pub fn test_send_xcm_as_nft(
            _: OriginFor<T>,
            verifier: xcm::latest::Junction,
            collection: xcm::latest::Junction,
            nft: xcm::latest::Junction,
            call: sp_std::vec::Vec<u8>,
        ) -> DispatchResult {
            let interior = xcm::latest::Junctions::X3(verifier, collection, nft);

            let dest = xcm::latest::MultiLocation {
                parents: 1,
                interior: xcm::latest::Junctions::X1(xcm::latest::Junction::Parachain(2125)),
            };

            let message = xcm::latest::Xcm(sp_std::vec![xcm::latest::Instruction::Transact {
                origin_kind: xcm::latest::OriginKind::Native,
                require_weight_at_most: xcm::latest::Weight::from_parts(50000000, 10000),
                call: <xcm::DoubleEncoded<_> as From<sp_std::vec::Vec<u8>>>::from(call),
            }]);

            pallet_xcm::Pallet::<T>::send_xcm(interior, dest, message)
                .map_err(|_| Error::<T>::SendingFailed)?;

            Ok(())
        }

        #[pallet::call_index(92)]
        #[pallet::weight(1)]
        pub fn test_send_xcm_as_verifier(
            _: OriginFor<T>,
            verifier: xcm::latest::Junction,
            call: sp_std::vec::Vec<u8>,
        ) -> DispatchResult {
            let interior = xcm::latest::Junctions::X1(verifier);

            let dest = xcm::latest::MultiLocation {
                parents: 1,
                interior: xcm::latest::Junctions::X1(xcm::latest::Junction::Parachain(2125)),
            };

            let message = xcm::latest::Xcm(sp_std::vec![xcm::latest::Instruction::Transact {
                origin_kind: xcm::latest::OriginKind::Native,
                require_weight_at_most: xcm::latest::Weight::from_parts(50000000, 10000),
                call: <xcm::DoubleEncoded<_> as From<sp_std::vec::Vec<u8>>>::from(call),
            }]);

            pallet_xcm::Pallet::<T>::send_xcm(interior, dest, message)
                .map_err(|_| Error::<T>::SendingFailed)?;

            Ok(())
        }
    }
}
