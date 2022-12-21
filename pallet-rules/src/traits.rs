use codec::{Decode, Encode};
use core::fmt::Debug;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub trait Rule {
    type CallRule: Encode + Decode + TypeInfo + Debug + Clone + PartialEq + Eq;

    type RuleSet: Encode
        + Decode
        + TypeInfo
        + Clone
        + PartialEq
        + Eq
        + RulesetManagement<Self::CallRule>;

    fn check_rule(&self, rule_set: &Self::RuleSet) -> bool;
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum RuleWrapper<RuleType> {
    Simple(bool),
    Rule(RuleType),
}

pub trait RulesetManagement<CallRule> {
    fn new() -> Self;

    fn add_rule(&mut self, rule: CallRule);
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

pub trait ToMainId {
    type MainId;

    fn to_main_id(&self) -> Self::MainId;
}
