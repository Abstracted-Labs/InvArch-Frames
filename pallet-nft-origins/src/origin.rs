use crate::{
    location::{Chain, NftLocation},
    pallet, Config,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{error::BadOrigin, RuntimeDebug};
use scale_info::TypeInfo;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum NftOrigin {
    Nft(NftLocation),
    Verifier(Chain),
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

pub fn ensure_verifier<T: Config, OuterOrigin>(o: OuterOrigin) -> Result<Chain, BadOrigin>
where
    OuterOrigin: Into<Result<pallet::Origin, OuterOrigin>>,
{
    match o.into() {
        Ok(NftOrigin::Verifier(chain)) => Ok(chain),
        _ => Err(BadOrigin),
    }
}
