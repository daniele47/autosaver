use crate::{
    fs::rel::RelPathStr,
    prof::{composite::Composite, module::Module, runner::Runner},
};

pub mod composite;
pub mod module;
pub mod parser;
pub mod runner;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Composite(Composite),
    Module(Module),
    Runner(Runner),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    name: RelPathStr,
    id: RelPathStr,
    kind: ProfileKind,
}

impl Profile {
    pub fn new(name: RelPathStr, id: RelPathStr, kind: ProfileKind) -> Self {
        Self { name, id, kind }
    }

    pub fn name(&self) -> &RelPathStr {
        &self.name
    }

    pub fn id(&self) -> &RelPathStr {
        &self.id
    }

    pub fn kind(&self) -> &ProfileKind {
        &self.kind
    }
}
