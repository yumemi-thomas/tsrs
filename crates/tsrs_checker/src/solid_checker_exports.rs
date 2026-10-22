// SOLID-CHECKER FORK: this file is not part of upstream tsrs.
//
// Public wrappers for two crate-private checker functions that the
// solid-checker Type Facts producer needs (TypeScript-Go equivalents were
// reached with go:linkname). Both are used by its `untypedCallable` sub-fact
// (callableWithoutSignatures). Keep this file the fork's only addition besides
// its `mod` line in lib.rs; rebase it onto each pinned upstream revision.
// https://github.com/yumemi-thomas/solid-checker (apps/solid-typefacts-rs)

use crate::*;

impl Checker {
    // checker.go isUntypedFunctionCall
    pub fn is_untyped_function_call_exported(
        &mut self,
        func_type: P<Type>,
        apparent_func_type: P<Type>,
        num_call_signatures: usize,
        num_construct_signatures: usize,
    ) -> bool {
        self.is_untyped_function_call(
            func_type,
            apparent_func_type,
            num_call_signatures,
            num_construct_signatures,
        )
    }

    // checker.go getReducedApparentType
    pub fn get_reduced_apparent_type_exported(&mut self, t: P<Type>) -> P<Type> {
        self.get_reduced_apparent_type(t)
    }
}
