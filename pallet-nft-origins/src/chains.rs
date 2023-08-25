use crate::location::Chain;
use xcm::latest::Junction;

pub trait ChainVerifier {
    fn get_chain_from_verifier(para_id_part: u32, verifier_part: Junction) -> Option<Chain>;
}
