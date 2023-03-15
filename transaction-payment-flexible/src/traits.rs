use codec::MaxEncodedLen;
use frame_support::{weights::Weight, Parameter};

pub trait FeeAssets<AssetId, Call>: Parameter + MaxEncodedLen {
    fn default_asset() -> AssetId;
    fn can_be_used(asset_id: AssetId, call: &Call) -> AssetId;
    fn get_asset_id(&self) -> AssetId;
}
