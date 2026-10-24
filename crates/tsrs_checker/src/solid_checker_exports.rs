// SOLID-CHECKER FORK: this file is not part of upstream tsrs.
//
// Public wrappers for crate-private checker functions that the solid-checker
// Type Facts producer needs (TypeScript-Go equivalents were reached with
// go:linkname). The first two are used by its `untypedCallable` sub-fact
// (callableWithoutSignatures); the rest by its checker-independent keys of
// derived symbols (apps/solid-typefacts/src/derived_key.rs). Each is a pure
// forward, listed with why. Keep this file the fork's only addition besides
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

    // checker.go couldContainTypeVariables. The producer keys an instantiated
    // symbol by its root when instantiation could not affect it, exactly as
    // `is_symbol_unaffected_by_instantiation` decides once the root's type is
    // resolved, so every checker gives one logical symbol one id.
    pub fn could_contain_type_variables_exported(&mut self, t: P<Type>) -> bool {
        self.could_contain_type_variables(t)
    }

    // tsrs-only (no Go equivalent): every tuple target shares one element
    // type parameter per index (`tuple_elements`), so a parameter's index is
    // its identity in every checker; the producer keys it by that index.
    pub fn tuple_element_index_exported(&self, t: P<Type>) -> Option<usize> {
        self.tuple_elements.iter().position(|(parameter, _)| parameter.addr() == t.addr())
    }

    // checker.go getWriteTypeOfSymbol. `is_symbol_unaffected_by_instantiation`
    // returns a set accessor itself only when its write type, cached on its
    // links once resolved, could not contain type variables either; the
    // producer resolves it the same way to key such an instantiation by its
    // root in every checker.
    pub fn get_write_type_of_symbol_exported(&mut self, symbol: P<Symbol>) -> Option<P<Type>> {
        self.get_write_type_of_symbol(symbol)
    }
}
