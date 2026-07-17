use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeEntry {
    pub child: RelPathStr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composite {
    pub entries: Vec<CompositeEntry>,
}
