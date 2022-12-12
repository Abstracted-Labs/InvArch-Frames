use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub trait Rule {
    type CallRule: Encode + Decode + TypeInfo + Debug + Clone + PartialEq + Eq;

    fn check_rule(&self, rule: Self::CallRule) -> bool;
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum CompRule<P> {
    Any,
    AnyOf(Vec<P>),
    NoneOf(Vec<P>),
    Exactly(P),
    GreaterThan(P),
    LesserThan(P),
    GreaterOrEqualTo(P),
    LesserOrEqualTo(P),
}

pub trait Process<P> {
    fn process(&self, compare_with: &P) -> bool;
}

impl<P: PartialEq + Eq + PartialOrd + Ord> Process<P> for CompRule<P> {
    fn process(&self, compare_with: &P) -> bool {
        match self {
            Self::Any => true,
            Self::AnyOf(vec) => vec.contains(compare_with),
            Self::NoneOf(vec) => !vec.contains(compare_with),
            Self::Exactly(v) => v == compare_with,
            Self::GreaterThan(v) => compare_with > v,
            Self::LesserThan(v) => compare_with < v,
            Self::GreaterOrEqualTo(v) => compare_with >= v,
            Self::LesserOrEqualTo(v) => compare_with <= v,
        }
    }
}

impl<P: PartialEq + Eq> Process<P> for CompRule<P> {
    default fn process(&self, compare_with: &P) -> bool {
        match self {
            Self::Any => true,
            Self::AnyOf(vec) => vec.contains(compare_with),
            Self::NoneOf(vec) => !vec.contains(compare_with),
            Self::Exactly(v) => v == compare_with,
            _ => false,
        }
    }
}
