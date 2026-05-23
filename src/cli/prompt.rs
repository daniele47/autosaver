use std::ops::{BitOr, BitOrAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptFlag {
    Yes = 1 << 0,
    No = 1 << 1,
    Edit = 1 << 2,
    Quit = 1 << 3,
    Diff = 1 << 4,
    Show = 1 << 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptFlags {
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prompt {
    allowed_flags: PromptFlags,
}

impl PromptFlags {
    fn new(flags: u32) -> Self {
        Self { flags }
    }

    pub fn empty() -> Self {
        Self::new(0)
    }

    fn contains(&self, flag: PromptFlag) -> bool {
        (self.flags & flag as u32) != 0
    }
}

impl From<PromptFlag> for PromptFlags {
    fn from(value: PromptFlag) -> Self {
        Self::new(value as u32)
    }
}

impl BitOr for PromptFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.flags | rhs.flags)
    }
}
impl BitOr<PromptFlag> for PromptFlags {
    type Output = Self;

    fn bitor(self, rhs: PromptFlag) -> Self::Output {
        self | PromptFlags::from(rhs)
    }
}
impl BitOr<PromptFlags> for PromptFlag {
    type Output = PromptFlags;

    fn bitor(self, rhs: PromptFlags) -> Self::Output {
        PromptFlags::from(self) | rhs
    }
}

impl BitOrAssign for PromptFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
impl BitOrAssign<PromptFlag> for PromptFlags {
    fn bitor_assign(&mut self, rhs: PromptFlag) {
        *self = *self | rhs;
    }
}
