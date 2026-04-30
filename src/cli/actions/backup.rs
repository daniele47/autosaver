use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
        output::Renderer,
    },
    core::profile::composite::ProfileLoader,
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
        let flag_notdiff = self.args.flags().contains(&Flag::Word("notdiff".into()));
        let flag_track = self.args.flags().contains(&Flag::Word("track".into()));

        let mut profile_loader = Self::profile_loader()?;
        let profile = profile_loader.load("test")?;

        todo!()
    }
}
