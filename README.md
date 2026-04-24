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

Note: modules and profiles will be for semplicity be referred to as profiles, since modules are techinically
a special profile with a single module in it (aka itself)

Ideas:
    - shebang like first line in config files, to differenciate between profiles and modules
    - profiles could themselves include other profiles too! (decide how to handle recursion eventually!)
    - use proper subcommands instead of flags: save / restore / ... (flags only for config things, like specifying the profile to use)
    - --help for flags/commands, --docs for ENTIRE cli manual, explaining everything

## todo

- [x] in fs module, add list_files and all_files functions
- [x] add proper and comprehensive tests for fs module
- [x] make list_files and all_files return BTreeSet which is always sorted!
- [x] make error more context aware
- [x] finish implementing resolve method in profile.rs
- [x] resolve seems to work, BUT error result is broken (return entire cycle)
- [x] implement tests for resolve method in profile.rs
- [ ] implement modules and profiles parsers
- [ ] implement resolve and extend functions in module.rs
- [ ] for resolve and extend functions in module.rs implement tests
