#[macro_export]
macro_rules! build_call_rules {
    ($($pallet:ident { $($name:ident { $($field:ident : $typ:ty),* $(,)* }),* $(,)* }),* $(,)*)  => {
        paste! {
            $(
                #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
                pub enum [<CallRules $pallet>] {
                    $($name { $( $field : CompRule<$typ>, )* },)*
                }
            )*

                #[derive(Encode, Decode, TypeInfo, Debug, Clone, PartialEq, Eq)]
            pub enum CallRules {
                $($pallet([<CallRules $pallet>]),)*
            }
        }

    };
}
