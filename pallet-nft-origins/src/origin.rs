use crate::{
    location::NftLocation,
    pallet::{self, Origin},
    Config,
};
use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use frame_support::{error::BadOrigin, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::traits::AtLeast32BitUnsigned;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub struct NftOrigin(pub NftLocation);

pub fn ensure_nft<T: Config, OuterOrigin>(o: OuterOrigin) -> Result<NftLocation, BadOrigin>
where
    OuterOrigin: Into<Result<pallet::Origin, OuterOrigin>>,
{
    match o.into() {
        Ok(NftOrigin(internal)) => Ok(internal),
        _ => Err(BadOrigin),
    }
}
