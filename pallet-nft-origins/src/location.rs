use crate::{
    chains::ChainVerifier,
    pallet::{self, Origin},
    Config,
};
use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use frame_support::{error::BadOrigin, RuntimeDebug};
use scale_info::TypeInfo;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{AtLeast32BitUnsigned, TrailingZeroInput};
use xcm::latest::{Junction, NetworkId};

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum Chain {
    Relay(NetworkId),
    Parachain(u32),
}

impl Chain {
    pub fn new_parachain_verified<T: Config>(
        para_id: u32,
        verifier_junction: Junction,
    ) -> Option<Self> {
        <<T as Config>::Chains as ChainVerifier>::get_chain_from_verifier(
            para_id,
            verifier_junction,
        )
    }
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum Collection {
    /// Pallet based NFT collection
    Id(u128),
    /// EVM based NFT collection
    Contract20([u8; 20]),
    /// WASM based NFT collection
    Contract32([u8; 32]),
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum Nft {
    /// Integer NFT id
    Id(u128),
    /// 20 bytes NFT id
    Key20([u8; 20]),
    /// 32 bytes NFT id
    Key32([u8; 32]),
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub struct NftLocation {
    /// Chain where the collection and NFT originate
    pub chain: Chain,
    /// NFT collection
    pub collection: Collection,
    /// Specific NFT
    pub nft: Nft,
}

impl NftLocation {
    pub fn new_verified<T: Config>(
        para_id: u32,
        verifier_junction: Junction,
        collection: Collection,
        nft: Nft,
    ) -> Option<Self> {
        <<T as Config>::Chains as ChainVerifier>::get_chain_from_verifier(
            para_id,
            verifier_junction,
        )
        .map(|chain| NftLocation {
            chain,
            collection,
            nft,
        })
    }

    pub fn new(chain: Chain, collection: Collection, nft: Nft) -> Self {
        Self {
            chain,
            collection,
            nft,
        }
    }

    pub fn derive_account<AccountId: Decode>(&self) -> AccountId {
        let chain = match self.chain {
            Chain::Relay(network_id) => (b"relay", network_id).encode(),
            Chain::Parachain(para_id) => (b"para", para_id).encode(),
        };

        let collection = match self.collection {
            Collection::Id(id) => (b"id", id).encode(),
            Collection::Contract20(key) => (b"contract20", key).encode(),
            Collection::Contract32(key) => (b"contract32", key).encode(),
        };

        let nft = match self.nft {
            Nft::Id(id) => (b"id", id).encode(),
            Nft::Key20(key) => (b"key20", key).encode(),
            Nft::Key32(key) => (b"key32", key).encode(),
        };

        let entropy = blake2_256(&[chain, collection, nft].concat());

        AccountId::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }
}
