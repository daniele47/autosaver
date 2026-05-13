use derive_getters::{Dissolve, Getters};

use crate::fs::path::PathStr;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Dissolve)]
pub struct RelPathStr {
    path: PathStr,
}
