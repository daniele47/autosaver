use std::{
    fs::{self, DirEntry, File, ReadDir},
    io::Read,
    path::Component,
};

use anyhow::{Context, bail};

use crate::fs::abs::AbsPathStr;

pub mod abs;
pub mod path;
pub mod rel;

#[derive(Debug)]
pub struct FindCtx {
    pub path: AbsPathStr,
    pub entry: DirEntry,
    pub depth: usize,
}

impl FindCtx {
    pub fn new(path: AbsPathStr, entry: DirEntry, depth: usize) -> Self {
        Self { path, entry, depth }
    }
}

impl AbsPathStr {
    fn list_raw(&self) -> anyhow::Result<ReadDir> {
        fs::read_dir(self.path()).with_context(|| {
            let p = self.display();
            format!("Could not list files in directory {p}")
        })
    }

    pub fn list<F>(&self, mut on_each: F) -> anyhow::Result<()>
    where
        F: FnMut(FindCtx) -> anyhow::Result<()>,
    {
        self.list_raw()?.try_for_each(|e| {
            let e = e?;
            let abs = AbsPathStr::new_from_pathbuf(e.path())?;
            on_each(FindCtx::new(abs, e, 1))
        })
    }

    pub fn find<F>(&self, mut on_each: F) -> anyhow::Result<()>
    where
        F: FnMut(FindCtx) -> anyhow::Result<bool>,
    {
        let mut stack: Vec<FindCtx> = vec![];
        let mut root_traversed = false;
        let mut children;
        let mut depth;

        loop {
            let item = stack.pop();
            if !root_traversed {
                children = self.list_raw()?;
                root_traversed = true;
                depth = 1;
            } else if let Some(ctx) = item {
                children = ctx.path.list_raw()?;
                depth = ctx.depth + 1;
                if !on_each(ctx)? {
                    continue;
                }
            } else {
                break;
            }

            children.try_for_each(|dir_entry| {
                let dir_entry = dir_entry?;
                let abs = AbsPathStr::new_from_pathbuf(dir_entry.path())?;
                let ftype = dir_entry.file_type()?;
                let is_dir = ftype.is_dir() || (ftype.is_symlink() && abs.path().is_dir());
                let ctx = FindCtx::new(abs, dir_entry, depth);
                if is_dir {
                    stack.push(ctx);
                } else {
                    on_each(ctx)?;
                }
                anyhow::Ok(())
            })?;
        }

        Ok(())
    }

    pub fn all_files<F>(self, mut on_each: F) -> anyhow::Result<()>
    where
        F: FnMut(Self) -> anyhow::Result<()>,
    {
        if self.is_file() {
            on_each(self)?;
        } else if self.is_dir() {
            self.find(|ctx| {
                let ftype = ctx.entry.file_type()?;
                if ftype.is_file() || (ftype.is_symlink() && ftype.is_file()) {
                    on_each(ctx.path)?;
                }
                Ok(true)
            })?;
        }
        Ok(())
    }

    pub fn all_files_ord(self, ord_files: &mut Vec<AbsPathStr>) -> anyhow::Result<()> {
        self.all_files(|p| {
            ord_files.push(p);
            Ok(())
        })?;
        ord_files.sort_unstable();
        Ok(())
    }

    pub fn purge_path_opts(&self, allow_recursive_delete: bool) -> anyhow::Result<()> {
        // skip if path not exist
        if self.path().symlink_metadata().is_err() {
            return Ok(());
        }

        // purge symlink
        if self.path().symlink_metadata().is_ok_and(|f| f.is_symlink()) {
            fs::remove_file(self.path()).with_context(|| {
                let p = self.display();
                format!("Could not delete symlink: {p}")
            })?;
        }
        // purge file
        else if self.is_file() {
            fs::remove_file(self.path()).with_context(|| {
                let p = self.display();
                format!("Could not delete file: {p}")
            })?;
        }
        // purge directory
        else if self.is_dir() {
            if allow_recursive_delete {
                fs::remove_dir_all(self.path()).with_context(|| {
                    let p = self.display();
                    format!("Could not delete directory recursively: {p}")
                })?;
            } else {
                fs::remove_dir(self.path()).with_context(|| {
                    let p = self.display();
                    format!("Could not delete directory: {p}")
                })?;
            }
        }
        // fail if it was something else
        else {
            let p = self.display();
            bail!("Could not delete path: {p}");
        }

        // delete empty parent directories
        let mut parent = self.path().parent();
        while let Some(p) = parent {
            if fs::remove_dir(p).is_err() {
                break;
            }
            parent = p.parent();
        }

        Ok(())
    }
    pub fn purge_path(&self) -> anyhow::Result<()> {
        self.purge_path_opts(false)
    }

    pub fn create_file(&self) -> anyhow::Result<()> {
        if self.is_file() {
            return Ok(());
        }

        // valid file can be created
        if !matches!(
            self.path().components().next_back(),
            Some(Component::Normal(_))
        ) {
            let p = self.display();
            bail!("Path cannot be created as a file: {p}")
        }

        // create parent dirs
        if let Some(parent) = self.path().parent() {
            fs::create_dir_all(parent).with_context(|| {
                let p = parent.display();
                format!("Failed to create directory: {p}")
            })?;
        } else {
            let p = self.display();
            bail!("Could not create parent directories: {p}");
        }

        // create file
        File::create(self.path()).with_context(|| {
            let p = self.display();
            format!("Failed to create file: {p}")
        })?;

        Ok(())
    }

    pub fn read_file(&self) -> anyhow::Result<String> {
        if !self.is_file() {
            let p = self.display();
            bail!("Cannot read a path that is not a file: {p}");
        }
        fs::read_to_string(self.path()).with_context(|| {
            let p = self.display();
            format!("Could not read file: {p}")
        })
    }

    pub fn copy_file(&self, dst: &Self) -> anyhow::Result<()> {
        dst.create_file()?;
        fs::copy(self.path(), dst.path()).with_context(|| {
            let p = self.display();
            let t = dst.display();
            format!("Failed to copy from {p} to {t}")
        })?;
        Ok(())
    }

    pub fn files_eq(&self, other: &Self) -> bool {
        || -> anyhow::Result<()> {
            let sm = self.path().metadata()?;
            let om = other.path().metadata()?;

            // check both paths are files
            if !sm.is_file() || !om.is_file() {
                bail!("Not files");
            }

            // check file len for faster checks
            if sm.len() != om.len() {
                bail!("Length differs");
            }

            // chunked byte comparison (works for both text and binary)
            let mut file1 = File::open(self.path())?;
            let mut file2 = File::open(other.path())?;

            let mut buf1 = [0; 8192];
            let mut buf2 = [0; 8192];

            loop {
                let n1 = file1.read(&mut buf1)?;
                let n2 = file2.read(&mut buf2)?;

                if n1 != n2 || buf1[..n1] != buf2[..n2] {
                    bail!("Chunk differs");
                }
                if n1 == 0 {
                    return Ok(());
                }
            }
        }()
        .is_ok()
    }
}
