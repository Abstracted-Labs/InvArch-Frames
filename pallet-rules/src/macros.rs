pub use codec::{Decode, Encode};
pub use paste::paste;
pub use scale_info::TypeInfo;

#[macro_export]
macro_rules! build_call_rules {
    ( $runtime_call:ident, $( $pallet_module:path, $pallet:ident { $($function:ident { $($field:ident : $typ:ty),* $(,)* }),* $(,)* }),* $(,)*)  => {

        pub use _call_rules_macro_internal_module::*;
        mod _call_rules_macro_internal_module {

            use super::*;

            use $crate::macros::{paste, Encode, Decode, TypeInfo};
            use $crate::traits::{Rule, CompRule, Process, RulesetManagement, RuleWrapper};

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

                $(
                    $(
                        #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
                        pub struct [<CallRules $pallet $function:camel>] {
                            $( $field : CompRule<$typ>, )*
                        }
                    )*
                )*

                #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq, Default)]
                pub struct RuleSet {
                    $(
                        $(
                            [<$pallet:lower _ $function>] : Option<[<CallRules $pallet $function:camel>]>,
                        )*
                    )*
                }

                impl RulesetManagement<CallRules> for RuleSet {

                    fn new() -> Self {
                        RuleSet::default()
                    }

                    fn add_rule(&mut self, rule: CallRules) {
                        match rule {
                            $(
                                $(
                                CallRules:: $pallet ( [<CallRules $pallet>] :: $function { $( $field , )* }) => {
                                        self.[<$pallet:lower _ $function>] = Some( [<CallRules $pallet $function:camel>] {
                                            $( $field , )*
                                        })
                                    },
                                )*
                            )*
                        }
                    }
                }

                impl Rule for $runtime_call {
                    type CallRule = CallRules;
                    type RuleSet = RuleSet;

                    fn check_rule(&self, rule_set: &Self::RuleSet) -> bool {
                        match self {

                            $(
                                $(
                                    $runtime_call :: $pallet ($pallet_module ::Call:: $function { $( $field , )* })
                                        => {
                                            if let Some( rule_set ) = &rule_set.[<$pallet:lower _ $function>] {
                                                $( rule_set. $field  .process( & $field ) )&&*
                                            } else { false } }
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
