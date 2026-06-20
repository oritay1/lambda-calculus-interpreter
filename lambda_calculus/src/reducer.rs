use std::collections::HashSet;
use crate::parser::Term;
use crate::utils;

/// Returns the set of free variables in a term.
fn fv(t: &Term) -> HashSet<String> {
    // please insert your code here
    match t {
        // Base condition: FV(x) = {x}
        Term::Variable(x) => {
            let mut set = HashSet::new(); // Create an empty set
            set.insert(x.clone());        // Insert a copy of x to the set
            set                           // Return the set
        }
        // Recursion 1: FV(\x.body) = FV(body) - {x}
        Term::Abstraction(x,body) => {
            let mut xbody = fv(body); // Calculate: FV(body)
            xbody.remove(x);          // Calculate: FV(body) - {x}
            xbody                     // Return: FV(body) - {x}
        }

        // Recursion 2: FV(t1 t2) = FV(t1) + FV(t2)
        Term::Application(t1, t2) => {
            let mut set1 = fv(t1); // Calculate set1: FV(t1)
            let set2 = fv(t2);     // Calculate set2: FV(t2)
            set1.extend(set2);     // Calculate (union set): FV(t1) + FV(t2)
            set1                   // Return the union set
        }
    }
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