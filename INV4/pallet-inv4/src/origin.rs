use crate::{
    multisig::{MultisigMember, MultisigMemberOf},
    pallet::{self, Origin},
    util::derive_core_account,
    Config,
};
use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use frame_support::{error::BadOrigin, RuntimeDebug};
use scale_info::TypeInfo;
use sp_runtime::traits::AtLeast32BitUnsigned;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum INV4Origin<
    T: pallet::Config,
    CoreId: AtLeast32BitUnsigned + Encode,
    AccountId: Decode + Encode + Clone,
> {
    Multisig(MultisigInternalOrigin<T, CoreId, AccountId>),
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub struct MultisigInternalOrigin<
    T: pallet::Config,
    CoreId: AtLeast32BitUnsigned + Encode,
    AccountId: Decode + Encode + Clone,
> {
    pub id: CoreId,
    t: PhantomData<(T, AccountId)>,
}

impl<
        T: pallet::Config,
        CoreId: AtLeast32BitUnsigned + Encode,
        AccountId: Decode + Encode + Clone,
    > MultisigInternalOrigin<T, CoreId, AccountId>
{
    pub fn new(id: CoreId) -> Self {
        Self { id, t: PhantomData }
    }

    pub fn to_account_id(&self) -> AccountId {
        derive_core_account::<T, CoreId, AccountId>(self.id.clone())
    }
}

pub fn ensure_multisig<T: Config, OuterOrigin>(
    o: OuterOrigin,
) -> Result<
    MultisigInternalOrigin<
        T,
        <T as pallet::Config>::CoreId,
        <T as frame_system::Config>::AccountId,
    >,
    BadOrigin,
>
where
    OuterOrigin: Into<Result<pallet::Origin<T>, OuterOrigin>>,
{
    match o.into() {
        Ok(Origin::<T>::Multisig(internal)) => Ok(internal),
        _ => Err(BadOrigin),
    }
}

pub fn ensure_multisig_member_account_id<T: Config, OuterOrigin>(
    o: OuterOrigin,
) -> Result<MultisigMemberOf<T>, BadOrigin>
where
    OuterOrigin: Into<Result<frame_system::Origin<T>, OuterOrigin>>,
{
    if let Ok(frame_system::Origin::<T>::Signed(account_id)) = o.into() {
        Ok(MultisigMember::<<T as frame_system::Config>::AccountId>::AccountId(account_id))
    } else {
        Err(BadOrigin)
    }
}

pub fn ensure_multisig_member_nft<T: Config, OuterOrigin>(
    o: OuterOrigin,
) -> Result<MultisigMemberOf<T>, BadOrigin>
where
    OuterOrigin: Into<Result<pallet_nft_origins::Origin, OuterOrigin>>,
{
    if let Ok(pallet_nft_origins::Origin::Nft(nft)) = o.into() {
        Ok(MultisigMember::<<T as frame_system::Config>::AccountId>::Nft(nft))
    } else {
        Err(BadOrigin)
    }
}
