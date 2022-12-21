use sp_runtime::{DispatchResult, DispatchResultWithInfo};
use sp_std::boxed::Box;

pub trait Permissions {
    type Id;
    type Call;

    fn check_permission(id: Self::Id, call: Box<Self::Call>) -> DispatchResultWithInfo<bool>;

    fn initialize_permission_set(id: Self::Id) -> DispatchResult;
}
