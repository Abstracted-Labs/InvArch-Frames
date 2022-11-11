#![cfg_attr(not(feature = "std"), no_std)]

use codec;
use codec::{Decode, Encode};
use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{Bounded, DispatchInfoOf, SaturatedConversion, Saturating, SignedExtension},
    transaction_validity::{
        InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
    },
};
use sp_std::{marker::PhantomData, prelude::*};
use wasm_instrument::parity_wasm::elements::{
    self, External, Func, FunctionType, Internal, MemoryType, Type, ValueType,
};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    // Import various types used to declare pallet in scope.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000_000_000)]
        pub fn new_rule(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;

            let module = RulesModule::new(&code).map_err(|_| Error::<T>::FailedBasicValidation)?;

            module
                .ensure_all_valid()
                .map_err(|_| Error::<T>::FailedRulesValidation)?;

            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        FailedBasicValidation,
        FailedRulesValidation,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}
}

struct RulesModule {
    /// A deserialized module. The module is valid (this is Guaranteed by `new` method).
    module: elements::Module,
}

impl RulesModule {
    fn new(original_code: &[u8]) -> Result<Self, &'static str> {
        use wasmi_validation::{validate_module, PlainValidator};

        let module =
            elements::deserialize_buffer(original_code).map_err(|_| "Can't decode wasm code")?;

        // Make sure that the module is valid.
        validate_module::<PlainValidator>(&module, ()).map_err(|_| "Module is not valid")?;

        // Return a `ContractModule` instance with
        // __valid__ module.
        Ok(RulesModule { module })
    }

    fn ensure_all_valid(&self) -> Result<(), &'static str> {
        self.ensure_no_internal_memory()?;
        self.ensure_no_global_variable()?;
        self.ensure_only_memory_import()?;
        self.ensure_no_floating_types()?;
        let func = self.ensure_single_function_defined()?;
        let ty = self.ensure_single_type_defined()?;
        self.ensure_single_export_correct_name()?;

        self.ensure_correct_function_type(ty)?;

        Ok(())
    }

    fn ensure_no_internal_memory(&self) -> Result<(), &'static str> {
        if self
            .module
            .memory_section()
            .map_or(false, |ms| ms.entries().len() > 0)
        {
            return Err("module declares internal memory");
        }
        Ok(())
    }

    fn ensure_no_global_variable(&self) -> Result<(), &'static str> {
        if let Some(global_section) = self.module.global_section() {
            if global_section.entries().len() > 0 as usize {
                return Err("module declares globals");
            }
        }
        Ok(())
    }

    fn ensure_only_memory_import(&self) -> Result<(), &'static str> {
        if let Some(import_section) = self.module.import_section() {
            let entries = import_section.entries();

            if entries.len() != 1 {
                return Err("Not exactly 1 import entry.");
            }

            let entry = &entries[0];

            if entry.module() != "e" || entry.field() != "m" {
                return Err("Wrong import entry.");
            }
        } else {
            return Err("Import section missing.");
        }

        Ok(())
    }

    /// Ensures that no floating point types are in use.
    fn ensure_no_floating_types(&self) -> Result<(), &'static str> {
        if let Some(global_section) = self.module.global_section() {
            for global in global_section.entries() {
                match global.global_type().content_type() {
                    ValueType::F32 | ValueType::F64 => {
                        return Err("use of floating point type in globals is forbidden")
                    }
                    _ => {}
                }
            }
        }

        if let Some(code_section) = self.module.code_section() {
            for func_body in code_section.bodies() {
                for local in func_body.locals() {
                    match local.value_type() {
                        ValueType::F32 | ValueType::F64 => {
                            return Err("use of floating point type in locals is forbidden")
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some(type_section) = self.module.type_section() {
            for wasm_type in type_section.types() {
                match wasm_type {
                    Type::Function(func_type) => {
                        let return_type = func_type.results().get(0);
                        for value_type in func_type.params().iter().chain(return_type) {
                            match value_type {
                                ValueType::F32 | ValueType::F64 => {
                                    return Err(
                                        "use of floating point type in function types is forbidden",
                                    )
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Ensure exactly one function exists.
    /// Returns function.
    fn ensure_single_function_defined(&self) -> Result<Func, &'static str> {
        let function_section = if let Some(function_section) = self.module.function_section() {
            function_section
        } else {
            return Err("No function section declared.");
        };

        let funcs = function_section.entries();

        if funcs.len() != 1 {
            return Err("Not exactly a single function defined.");
        }

        Ok(funcs[0])
    }

    /// Ensure exactly one type exists and is FunctionType.
    /// Returns FunctionType.
    fn ensure_single_type_defined(&self) -> Result<FunctionType, &'static str> {
        let type_section = if let Some(type_section) = self.module.type_section() {
            type_section.types()
        } else {
            return Err("No type section declared.");
        };

        // Type enum currently can only be Function, so we can just check if the length of type_section.types() is exactly 1.

        if type_section.len() != 1 {
            return Err("Not exactly a single function defined.");
        }

        let Type::Function(func_ty) = &type_section[0];

        Ok(func_ty.clone())
    }

    fn ensure_single_export_correct_name(&self) -> Result<(), &'static str> {
        if let Some(export_section) = self.module.export_section() {
            let entries = export_section.entries();

            if entries.len() != 1 {
                return Err("More than one export.");
            }

            if entries[0].field() != "f" {
                return Err("Incorrect function name");
            }
        }

        Ok(())
    }

    fn ensure_correct_function_type(&self, ty: FunctionType) -> Result<(), &'static str> {
        if !(ty.params().is_empty() && (ty.results() == [ValueType::I32])) {
            return Err("entry point has wrong signature");
        }

        Ok(())
    }
}
