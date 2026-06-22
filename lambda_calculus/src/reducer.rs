use std::collections::HashSet;
use std::ops::ControlFlow::Continue;
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
    // please insert your code here
    match t2 {
        
        // If t2 is a variable
        Term::Variable(y) => {
            // If t2 is the same variable as the variable we want to substitute, just return the substitution variable: t1
            if y == x {
                t1.clone()
            }
            // If t2 is a different variable that means x is not in that term so there is nothing to substitute so we return y (which is t2)
            else {
               Term::Variable(y.clone()) 
            }
        }

        Term::Abstraction(y,body) => {
            let free_variables = fv(t1);
            // Case 1:
            if y == x {
                t2.clone()
            }
            // Case 2:
            else if y != x && !free_variables.contains(y) {
                Term::Abstraction(y.clone(), Box::new(substitute(x, t1, body)))
            }

            // Case 3:
            else {
                let mut used_variables = fv(t1);
                used_variables.extend(fv(body));
                used_variables.insert(x.to_string());

                let z = utils::fresh_var(&used_variables);

                let first_sub = substitute(y, &Term::Variable(z.clone()), body);     
                let second_sub = substitute(x, t1, &first_sub);
                Term::Abstraction(z,Box::new(second_sub))
            }
        }

        Term::Application(term1, term2) => {
            let new_t1 = substitute(x, t1, term1);
            let new_t2 = substitute(x, t1, term2);
            Term::Application(Box::new(new_t1), Box::new(new_t2))
        }
    }
}

/// Call-by-Value reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbv(t: &Term) -> Option<Term> {
    // please insert your code here
    match t {
        Term::Variable(_y) => {
            return None;
        }

        Term::Abstraction(_y,_body) => {
            return None;
        }
        
        Term::Application(term1, term2) => {
            // Case 1: using E-App1 is always permitted
            if let Some(new_term1) = reduce_cbv(term1) {
                return Some(Term::Application(Box::new(new_term1), term2.clone()));
            }
            // Case 2: try use E-App2 if term1 is a value (abstraction) - use E-App2
            if is_value(term1) {
                if let Some(new_term2) = reduce_cbv(term2) {
                    return Some(Term::Application(term1.clone(), Box::new(new_term2)));
                }
            }
            // Case 3: E-AppAbs
            // If both term1 and term2 are values, perform substitution
            if is_value(term1) && is_value(term2) {
                // Extract the parameter 'x' and the inner 'body' from term1
                if let Term::Abstraction(x, body) = &**term1 {
                    let result = substitute(x, term2, body);
                    return Some(result);
                }
            }
            // If no rules apply, no reduction is possible
            None
        }
    }
}

/// Call-by-Name reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbn(t: &Term) -> Option<Term> {
    // please insert your code here
    match t {
        Term::Variable(_y) => {
            return None;
        }

        Term::Abstraction(_y,_body) => {
            return None;
        }
        
        Term::Application(term1, term2) => {
            // Case 1: using E-App1 is always permitted
            if let Some(new_term1) = reduce_cbn(term1) {
                return Some(Term::Application(Box::new(new_term1), term2.clone()));
            }

            // Case 2: E-AppAbs
            // If term1 is value, perform substitution
            if is_value(term1) {
                // Extract the parameter 'x' and the inner 'body' from term1
                if let Term::Abstraction(x, body) = &**term1 {
                    let result = substitute(x, term2, body);
                    return Some(result);
                }
            }
            // If no rules apply, no reduction is possible
            None
        }
    }
}

/// Helper function
fn is_value(t: &Term) -> bool {
    match t {
        Term::Abstraction(_, _) => true,
        _ => false,
    }
}