# dotfiles-rust

Copy-based dotfiles tracking cli, written in rust

## ideas

- dir structure:

```
dotfiles/
├── autosaver
├── .defaults
├── configs/
│   ├── module1.conf
│   ├── module2.conf
│   ├── profile1.conf
│   └── profile2.conf
└── backups/
    ├── module1/
    └── module2/
```

- module format:

```
/! type module

// policies have always as default
.config/nvim

// after the following line, policy becomes ignore, aka the files reported next will be not tracked,
// even if previous line would have added them, either directly (as files) or indirectly (as part of dirs)

/! policy ignore
.config/nvim/lazy-lock.json 
```

- profile format:

```
/! type profile

neovim
tmux
kde-plasma
```

- autosaver: bash wrapper script to get rust binary (downloaded/compiled) and run it
- .defaults: NOT TRACKED file to store default configurations, things like what module/profile to use by default
- configs: all modules and profile configurations, one config x file
- modules: simple list of files to track
- profiles: groups of modules to apply sequentially
- backups: each module has exactly one backup dir where to save its files, and named like the module

- modules and profiles example formats are reported above
    - // for comments
    - /! for special instruction lines
    - /<char> is extendable in the future, for now ignored!
    - also: spaces should be trimmed, since hopefully no app 
      is insane to use name with starting/ending whitespace

For example: if `neovim`, `tmux`, `plasma-desktop` are possible modules, 
`minimal-cli` or `kde-linux` are possible profiles and minimal-cli would
only have neovim and tmux, for example

## todo

- [x] in fs module, add `list_files` and `all_files` functions
- [x] add proper and comprehensive tests for fs module
- [x] make `list_files` and `all_files` return BTreeSet which is always sorted!
- [x] make error more context aware
- [x] finish implementing `resolve` method in profile.rs
- [x] resolve seems to work, BUT error result is broken (return entire cycle)
- [x] implement tests for `resolve` method in profile.rs
- [ ] implement modules and profiles `parsers`
- [x] implement `resolve` and `extend` functions in module.rs
- [x] for `resolve` and `extend` functions in module.rs implement tests
- [x] add function in `fs` module to do buffered reads (`BufReader` seems to implement a `.lines()` method!!!)
- [x] required way to convert `AbsPath` and `RelPath` to String (best idea: use TryFrom and an error variant!)
- [ ] `resolve` func needs to be careful of duplicates by equivalent names (.config/nvim vs .config/nvim/)
- [ ] `resolve_and_merge` function or smt, to allow resolving multiple Modules, and to merge results into a single module
- [ ] `resolve_and_merge` add test
- [ ] make sure to remove all print from all tests and code! just brutally grep to find them all!

### long term todos

#### cli ideas

- [ ] add `cleanup` action which acts like untracked FOR ALL MODULES + all possible cleanups,
      such as allow deleting backup dirs without a respective config file, check there are NO
      symlinks in the dotfiles repo, ...
- [ ] add `help` action to get direct help on possible args and flags to the cli
- [ ] add `doc` action to print an entire manual with all things to know about the script

- [ ] do i need `version` action? i doubt i should. No backward compatibility, KISS!
