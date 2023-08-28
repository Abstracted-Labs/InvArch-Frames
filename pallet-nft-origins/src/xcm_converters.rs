use crate::{
    location::{Chain, Collection, Nft, NftLocation},
    origin::NftOrigin,
    Config,
};
use core::marker::PhantomData;
use frame_support::traits::OriginTrait;
use xcm::latest::{
    Junction,
    Junctions::{X2, X4},
    MultiLocation, OriginKind,
};
use xcm_executor::traits::ConvertOrigin;

pub struct NftMultilocationAsOrigin<RuntimeOrigin, T>(PhantomData<(RuntimeOrigin, T)>);

impl<RuntimeOrigin: OriginTrait + From<crate::pallet::Origin>, T: Config>
    ConvertOrigin<RuntimeOrigin> for NftMultilocationAsOrigin<RuntimeOrigin, T>
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
                    parents: 1,
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
            ) => NftLocation::new_verified::<T>(
                para_id,
                verifier_junction,
                Collection::Contract20(collection_key),
                Nft::Id(nft_id),
            )
            .map(|location| NftOrigin::Nft(location).into())
            .ok_or(origin),

            (_, origin) => Err(origin),
        }
    }
}

pub struct VerifierMultilocationAsOrigin<RuntimeOrigin, T>(PhantomData<(RuntimeOrigin, T)>);

impl<RuntimeOrigin: OriginTrait + From<crate::pallet::Origin>, T: Config>
    ConvertOrigin<RuntimeOrigin> for VerifierMultilocationAsOrigin<RuntimeOrigin, T>
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
                    parents: 1,
                    interior: X2(Junction::Parachain(para_id), verifier_junction),
                },
            ) => Chain::new_parachain_verified::<T>(para_id, verifier_junction)
                .map(|chain| NftOrigin::Verifier(chain).into())
                .ok_or(origin),

            (_, origin) => Err(origin),
        }
    }
}
