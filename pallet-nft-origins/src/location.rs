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
use xcm::latest::Junction;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub enum Chain {
    /// Relay chain
    Relay,
    /// Parachain with ParaId
    Parachain(u32),
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
    pub fn new<T: Config>(
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

    pub fn derive_account<AccountId: Decode>(&self) -> AccountId {
        let chain = match self.chain {
            Chain::Relay => "relay".encode(),
            Chain::Parachain(para_id) => ["para-".encode(), para_id.encode()].concat(),
        };

        let collection = match self.collection {
            Collection::Id(id) => ["id-".encode(), id.encode()].concat(),
            Collection::Contract20(key) => ["contract20-".encode(), key.encode()].concat(),
            Collection::Contract32(key) => ["contract32-".encode(), key.encode()].concat(),
        };

        let nft = match self.nft {
            Nft::Id(id) => ["id-".encode(), id.encode()].concat(),
            Nft::Key20(key) => ["key20-".encode(), key.encode()].concat(),
            Nft::Key32(key) => ["key32-".encode(), key.encode()].concat(),
        };

        let entropy = (chain, collection, nft).using_encoded(blake2_256);

        AccountId::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }
}
