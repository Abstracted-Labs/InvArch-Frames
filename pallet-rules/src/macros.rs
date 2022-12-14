pub use codec::{Decode, Encode};
pub use paste::paste;
pub use scale_info::TypeInfo;

#[macro_export]
macro_rules! build_call_rules {
    ($( $pallet_module:path, $pallet:ident { $($function:ident { $($field:ident : $typ:ty),* $(,)* }),* $(,)* }),* $(,)*)  => {

        use call_rules::*;
        mod call_rules {

            use super::*;

            use $crate::macros::{paste, Encode, Decode, TypeInfo};
            use $crate::traits::{Rule, GetRuleId, CompRule, Process};

            paste! {
                $(
                    #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
                    pub enum [<CallRules $pallet>] {
                        $($function { $( $field : CompRule<$typ>, )* },)*
                    }
                )*

                    #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
                pub enum CallRules {
                    $($pallet([<CallRules $pallet>]),)*
                }

                #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
                pub enum CallIds {
                    $(
                        $([<$pallet $function:camel>],)*
                    )*
                }

                impl GetRuleId<CallIds> for CallRules {
                    fn get_id(&self) -> CallIds {
                        match self {
                            $(
                                $(
                                    CallRules:: $pallet ( [<CallRules $pallet>]:: $function { .. } ) => CallIds::[<$pallet $function:camel>],
                                )*
                            )*
                        }
                    }
                }

                impl Rule for Call {
                    type CallRule = CallRules;
                    type CallId = CallIds;

                    fn check_rule(&self, rule: Self::CallRule) -> bool {
                        match (self, rule) {


                            $(
                                $(

                                    (
                                        Call:: $pallet ($pallet_module ::Call:: $function { $( $field , )* }),
                                        CallRules:: $pallet ([<CallRules $pallet>] :: $function { $( $field : [<$field _r>], )* }),
                                    ) => { $( [<$field _r>].process( $field ) )&&* }
                                )*
                            )*

                                _ => false,
                        }
                    }
                }
            }
        }
    };
}
