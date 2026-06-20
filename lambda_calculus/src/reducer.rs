use std::collections::HashSet;
use crate::parser::Term;
use crate::utils;

/// Returns the set of free variables in a term.
fn fv(t: &Term) -> HashSet<String> {
    /// please insert your code here
}

/// Substitute: replace occurrences of `x` with `t1` inside `t2`.
/// Notation: t2 [x := t1] 
fn substitute(x: &str, t1: &Term, t2: &Term) -> Term {
    /// please insert your code here
}

/// Call-by-Value reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbv(t: &Term) -> Option<Term> {
       /// please insert your code here
}

/// Call-by-Name reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbn(t: &Term) -> Option<Term> {
       /// please insert your code here
}