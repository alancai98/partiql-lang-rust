use std::fmt::Debug;

use partiql_value::{BindingsName, Tuple, Value};
use unicase::UniCase;

pub trait Bindings<T> {
    fn get(&self, name: &BindingsName) -> Option<&T>;
}

impl Bindings<Value> for Tuple {
    fn get(&self, name: &BindingsName) -> Option<&Value> {
        match name {
            BindingsName::CaseSensitive(s) => self.0.get(s),
            BindingsName::CaseInsensitive(s) => {
                //TODO --TODO(ALAN) what's this TODO for? -- not handling case-insensitivity here yet
                self.0.get(s)
            }
        }
    }
}

//--TODO(ALAN) why is this module called 'basic'. Does this module provide the most basic of
//      bindings (i.e. use a `HashMap` for case(insensitive) bindings?. Will we provide other
//      binding types (e.g. one that supports duplicate insensitive
pub mod basic {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct MapBindings<T> {
        sensitive: HashMap<String, usize>, //--TODO(ALAN) returns usize representing the index from within the `values` Vec?
        insensitive: HashMap<UniCase<String>, usize>,   //(ALAN) -- UniCase is case-insensitive wrapper for `String`s
        values: Vec<T>,
    }

    impl<T> Default for MapBindings<T> {
        fn default() -> Self {
            MapBindings {
                sensitive: HashMap::new(),
                insensitive: HashMap::new(),
                values: vec![],
            }
        }
    }

    impl<T> MapBindings<T> {
        pub fn insert(&mut self, name: &str, value: T) {
            // TODO error on duplicate insensitive --TODO(ALAN) so we won't allow case-insensitive bindings in either
            let idx = self.values.len();
            self.values.push(value);
            self.sensitive.insert(name.to_string(), idx);
            self.insensitive.insert(UniCase::new(name.to_string()), idx);
        }
    }

    impl<T> Bindings<T> for MapBindings<T> {
        #[inline]
        fn get(&self, name: &BindingsName) -> Option<&T> {
            let idx = match name {
                BindingsName::CaseSensitive(s) => self.sensitive.get(s),
                BindingsName::CaseInsensitive(s) => {
                    self.insensitive.get(&UniCase::new(s.to_string()))
                }
            };
            idx.and_then(|idx| self.values.get(*idx))
        }
    }
}
