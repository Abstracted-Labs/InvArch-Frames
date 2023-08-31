use crate::{chains::ChainVerifier, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use primitive_types::U256;
use scale_info::TypeInfo;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::TrailingZeroInput;
use xcm::latest::Junction;

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub struct Parachain(pub u32);

impl Parachain {
    pub fn new_parachain_verified<T: Config>(
        para_id: u32,
        verifier_junction: Junction,
    ) -> Option<Self> {
        <<T as Config>::Chains as ChainVerifier>::get_chain_from_verifier(
            para_id,
            verifier_junction,
        )
    }

    pub const fn para_id(&self) -> u32 {
        self.0
    }
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
#[repr(u8)]
pub enum Collection {
    /// Pallet based NFT collection
    Id(u128) = 0,
    /// EVM based NFT collection
    Contract20([u8; 20]) = 1,
    /// WASM based NFT collection
    Contract32([u8; 32]) = 2,
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
#[repr(u8)]
pub enum Nft {
    /// U128 NFT id
    U128Id(u128) = 0,
    /// U256 NFT id
    U256Id(U256) = 1,
    /// 20 bytes NFT id
    Key20([u8; 20]) = 2,
    /// 32 bytes NFT id
    Key32([u8; 32]) = 3,
}

#[derive(PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen, Clone, RuntimeDebug)]
pub struct NftLocation {
    /// Chain where the collection and NFT originate
    pub chain: Parachain,
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

    pub fn new(chain: Parachain, collection: Collection, nft: Nft) -> Self {
        Self {
            chain,
            collection,
            nft,
        }
    }

    pub fn derive_account<AccountId: Decode>(&self) -> AccountId {
        let chain = (b"para", self.chain.para_id()).encode();

        let collection = match self.collection {
            Collection::Id(id) => (b"id", id).encode(),
            Collection::Contract20(key) => (b"contract20", key).encode(),
            Collection::Contract32(key) => (b"contract32", key).encode(),
        };

        let nft = match self.nft {
            Nft::U128Id(id) => (b"u128id", id).encode(),
            Nft::U256Id(id) => (b"u256id", id).encode(),
            Nft::Key20(key) => (b"key20", key).encode(),
            Nft::Key32(key) => (b"key32", key).encode(),
        };

        let entropy = blake2_256(&[chain, collection, nft].concat());

        AccountId::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }
}
