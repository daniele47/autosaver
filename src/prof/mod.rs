use crate::{
    fs::rel::RelPathStr,
    prof::{composite::Composite, module::Module, runner::Runner},
};

pub mod composite;
pub mod module;
pub mod parsers;
pub mod runner;

pub enum ProfileKind {
    Composite(Composite),
    Module(Module),
    Runner(Runner),
}

pub struct Profile {
    name: RelPathStr,
    id: RelPathStr,
    kind: ProfileKind,
}
