use crate::{
    chains::ChainVerifier,
    location::{Collection, Nft, NftLocation},
    origin::NftOrigin,
    Config,
};
use core::marker::PhantomData;
use frame_support::traits::OriginTrait;
use xcm::latest::{Junction, Junctions::X4, MultiLocation, OriginKind};
use xcm_executor::traits::ConvertOrigin;

pub struct NftMultiLocationAsOrigin<RuntimeOrigin, T>(PhantomData<(RuntimeOrigin, T)>);

impl<RuntimeOrigin: OriginTrait + From<crate::pallet::Origin>, T: Config>
    ConvertOrigin<RuntimeOrigin> for NftMultiLocationAsOrigin<RuntimeOrigin, T>
{
    fn convert_origin(
        origin: impl Into<MultiLocation>,
        kind: OriginKind,
    ) -> Result<RuntimeOrigin, MultiLocation> {
        let origin = origin.into();
        log::trace!(target: "xcm::origin_conversion", "ParentAsSuperuser origin: {:?}, kind: {:?}", origin, kind);

        match (kind, origin) {
            (
                OriginKind::Native,
                MultiLocation {
                    // Parents will match from the perspective of the relay or one of it's child parachains.
                    parents: 0 | 1,
                    interior:
                        X4(
                            Junction::Parachain(para_id),
                            verifier_junction,
                            Junction::AccountKey20 {
                                network: None,
                                key: collection_key,
                            },
                            Junction::GeneralIndex(nft_id),
                        ),
                },
            ) => NftLocation::new::<T>(
                para_id,
                verifier_junction,
                Collection::Contract20(collection_key),
                Nft::Id(nft_id),
            )
            .map(|location| NftOrigin(location).into())
            .ok_or(origin),

            (_, origin) => Err(origin),
        }
    }
}
