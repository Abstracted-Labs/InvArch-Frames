use crate::{
    location::{NftLocation, Parachain},
    pallet, Config,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{error::BadOrigin, RuntimeDebug};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum NftOrigin {
    Nft(NftLocation),
    Verifier(Parachain),
}

pub fn ensure_nft<T: Config, OuterOrigin>(o: OuterOrigin) -> Result<NftLocation, BadOrigin>
where
    OuterOrigin: Into<Result<pallet::Origin, OuterOrigin>>,
{
    match o.into() {
        Ok(NftOrigin::Nft(nft_location)) => Ok(nft_location),
        _ => Err(BadOrigin),
    }
}

pub fn ensure_verifier<T: Config, OuterOrigin>(o: OuterOrigin) -> Result<Parachain, BadOrigin>
where
    OuterOrigin: Into<Result<pallet::Origin, OuterOrigin>>,
{
    match o.into() {
        Ok(NftOrigin::Verifier(chain)) => Ok(chain),
        _ => Err(BadOrigin),
    }
}
