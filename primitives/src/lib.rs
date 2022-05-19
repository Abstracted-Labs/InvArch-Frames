#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::Percent;
use sp_std::{vec, vec::Vec};

type Hash = sp_core::H256;

pub trait LicenseList {
    type IpfsHash: core::hash::Hash;
    type LicenseMetadata;

    fn get_hash_and_metadata(&self) -> (Self::LicenseMetadata, Self::IpfsHash);
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, Eq, PartialEq)]
pub enum InvArchLicenses {
    Apache2,
    GPLv3,
    Custom(Vec<u8>, Hash),
}

impl LicenseList for InvArchLicenses {
    type IpfsHash = Hash; // License IPFS hash.
    type LicenseMetadata = Vec<u8>; // License name.

    fn get_hash_and_metadata(&self) -> (Self::LicenseMetadata, Self::IpfsHash) {
        match self {
            InvArchLicenses::Apache2 => (
                vec![65, 112, 97, 99, 104, 97, 32, 118, 50, 46, 48],
                [
                    7, 57, 92, 251, 234, 183, 217, 144, 220, 196, 201, 132, 176, 249, 18, 224, 237,
                    201, 2, 113, 146, 78, 111, 152, 92, 71, 16, 228, 87, 39, 81, 142,
                ]
                .into(),
            ),
            InvArchLicenses::GPLv3 => (
                vec![71, 78, 85, 32, 71, 80, 76, 32, 118, 51],
                [
                    72, 7, 169, 24, 30, 7, 200, 69, 232, 27, 10, 138, 130, 253, 91, 158, 210, 95,
                    127, 37, 85, 41, 106, 136, 66, 116, 64, 35, 252, 195, 69, 253,
                ]
                .into(),
            ),
            InvArchLicenses::Custom(metadata, hash) => (metadata.clone(), *hash),
        }
    }
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum OneOrPercent {
    One,
    ZeroPoint(Percent),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum Parentage<AccountId, IpsId> {
    /// Parent IP (Account Id of itself)
    Parent(AccountId),
    /// Child IP (Id of the immediate parent, Account Id of the topmost parent)
    Child(IpsId, AccountId),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum IpsType<IpsId> {
    /// Normal IPS
    Normal,
    /// IP Replica (Id of the original IP)
    Replica(IpsId),
}

/// IPS info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub struct IpsInfo<AccountId, Data, IpsMetadataOf, IpsId> {
    /// IPS parentage
    pub parentage: Parentage<AccountId, IpsId>,
    /// IPS metadata
    pub metadata: IpsMetadataOf,
    /// IPS Properties
    pub data: Data,
    /// IPS Type
    pub ips_type: IpsType<IpsId>,
    /// If this IPS allows replicas
    pub allow_replica: bool,
}

/// IPF Info
#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub struct IpfInfo<AccountId, Data, IpfMetadataOf> {
    /// IPF owner
    pub owner: AccountId,
    /// Original IPF author
    pub author: AccountId,
    /// IPF metadata
    pub metadata: IpfMetadataOf,
    /// IPF data
    pub data: Data,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct IptInfo<AccountId, Balance> {
    pub owner: AccountId,
    /// The total supply across all accounts.
    pub supply: Balance,
}

// This is a struct in preparation for having more fields in the future.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct SubIptInfo<IptId, SubAssetMetadata> {
    pub id: IptId,
    pub metadata: SubAssetMetadata,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct IplInfo<AccountId, IplId, LicenseMetadata, Hash> {
    pub owner: AccountId,
    pub id: IplId,
    pub license: (LicenseMetadata, Hash),
    pub execution_threshold: OneOrPercent,
    pub default_asset_weight: OneOrPercent,
    pub default_permission: bool,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub struct CallInfo<Data> {
    pub pallet: Data,
    pub function: Data,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub enum AnyId<IpsId, IpfId> {
    IpsId(IpsId),
    IpfId(IpfId),
}

pub mod utils {
    use codec::{Decode, Encode};
    use sp_io::hashing::blake2_256;
    use sp_runtime::traits::TrailingZeroInput;

    pub fn multi_account_id<T: frame_system::Config, IpsId: Encode>(
        ips_id: IpsId,
        original_caller: Option<T::AccountId>,
    ) -> <T as frame_system::Config>::AccountId {
        let entropy = if let Some(original_caller) = original_caller {
            (b"modlpy/utilisuba", ips_id, original_caller).using_encoded(blake2_256)
        } else {
            (b"modlpy/utilisuba", ips_id).using_encoded(blake2_256)
        };

        Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }
}
