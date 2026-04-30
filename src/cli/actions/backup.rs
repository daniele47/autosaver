use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
        output::Renderer,
    },
    core::profile::{ProfileType, composite::ProfileLoader},
};

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    pub fn backup(&mut self) -> Result<()> {
        let mut iter = self.args.params().iter();
        let arg_command = iter.next().map(String::as_str).unwrap_or_default();
        let arg_profile = iter.next().map(String::as_str).unwrap_or_default();
        let wflag_y = self.args.flags().contains(&Flag::Word("assumeyes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assumeno".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;
        let wflag_diff = self.args.flags().contains(&Flag::Word("diff".into()));
        let lflag_diff = self.args.flags().contains(&Flag::Letter('d'));
        let flag_diff = wflag_diff || lflag_diff;
        let wflag_all = self.args.flags().contains(&Flag::Word("all".into()));
        let lflag_all = self.args.flags().contains(&Flag::Letter('a'));
        let flag_all = wflag_all || lflag_all;

        let mut profile_loader = Self::profile_loader()?;
        let profile = profile_loader.load("test")?;

        // load all modules into a vector
        let mut modules = vec![];
        match profile.ptype() {
            ProfileType::Composite(composite) => {
                let resolved = composite.resolve(arg_profile, &mut profile_loader)?;
                for entry in resolved.entries() {
                    let p = profile_loader.load(entry)?;
                    match p.ptype() {
                        ProfileType::Composite(_) => {
                            unreachable!("Profile should be already solved")
                        }
                        ProfileType::Module(module) => {
                            modules.push(module.clone());
                        }
                    }
                }
            }
            ProfileType::Module(module) => {
                modules.push(module.clone());
            }
        }

        println!("{arg_command} {flag_y} {flag_n} {flag_diff} {flag_all}");

        todo!("Do operations on 1 module at a time")
    }
}
