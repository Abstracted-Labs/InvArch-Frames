#![cfg_attr(not(feature = "std"), no_std)]

pub mod chains;
pub mod location;
pub mod origin;
pub mod xcm_converters;

pub use chains::ChainVerifier;
pub use location::{Chain, Collection, Nft, NftLocation};
pub use origin::NftOrigin;
pub use xcm_converters::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{chains::ChainVerifier, origin::NftOrigin};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_xcm::Config {
        type Chains: ChainVerifier;

        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::origin]
    pub type Origin = NftOrigin;

    #[pallet::error]
    pub enum Error<T> {
        SendingFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

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
        #[pallet::weight(1)]
        pub fn test_nft_location(nft: OriginFor<T>) -> DispatchResult {
            let nft_location =
                crate::origin::ensure_nft::<T, <T as frame_system::Config>::RuntimeOrigin>(nft)?;

            Self::deposit_event(Event::<T>::NftCalledTestFunction { nft_location });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(1)]
        pub fn test_send_xcm_as(
            o: OriginFor<T>,
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
    }
}
