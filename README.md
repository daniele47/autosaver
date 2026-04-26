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

- For example: if `neovim`, `tmux`, `plasma-desktop` are possible modules, `minimal-cli` or `kde-linux` are possible 
  profiles and minimal-cli would only have neovim and tmux, for example

- logs for every operation?
    - I could have them shoved into .logs dir, and one file x command run with timestamp so can be easily ordered
    - Or they might just be for dangerous operations, aka probably only for all fs operations
    - I could even log crashes potentially?

- add versions:
    - use cargo OWN version (see `src/main/version.rs` for an easy example)
    - keep version `0.1.0` until i start compiling the first binaries
    - from then on, release a `1.0.0` and bump the version every time i release a new binary, following semver
    - in build directory, add a readme file to document entirely all the steps to release a new version
    - find out how to do codeberg releases properly, which will then be used to download binaries from my bash script

## universal rules

1. No `unwraps` in the code, use explicit `assertions to validate invariants`
2. Do `tests` only for important and complex functions that are hard to get right
3. Make sure tests `always cleanup resources` even on panic
4. Make sure there are `no weird debug print` ever left in code, except if meant as part of the cli
5. Add `more comments` to complex functions, just to give general ideas on what is going on

## todo

- [x] ~in fs module, add `list_files` and `all_files` functions~
- [x] ~add proper and comprehensive tests for fs module~
- [x] ~make `list_files` and `all_files` return BTreeSet which is always sorted!~
- [x] ~make error more context aware~
- [x] ~finish implementing `resolve` method in profile.rs~
- [x] ~resolve seems to work, BUT error result is broken (return entire cycle)~
- [x] ~implement tests for `resolve` method in profile.rs~
- [ ] implement modules and profiles `parsers`
- [x] ~implement `resolve` and `extend` functions in module.rs~
- [x] ~for `resolve` and `extend` functions in module.rs implement tests~
- [x] ~add function in `fs` module to do buffered reads (`BufReader` seems to implement a `.lines()` method!!!)~
- [x] ~required way to convert `AbsPath` and `RelPath` to String (best idea: use TryFrom and an error variant!)~
- [x] ~`resolve` func needs to be careful of duplicates by equivalent names (.config/nvim vs .config/nvim/)~
- [x] ~make sure to remove all print from all tests and code! just brutally grep to find them all!~
- [x] ~profile resolver might actually need to be a DFS instead of BFS!~
- [x] ~profile resolver needs to actually add modules to stack too! otherwise breaks!~
- [x] ~make cycle detection more powerful and detect 1 full cycle, for way better error msg! (use `Three-colors DFS`)~
- [x] ~make errors actually be a struct variant (aka {io: ..., path: ...} instead of (ioError,AbsPath)~
- [x] ~move `compile.sh` file into `builds/` itself (tweak logic to make it still work + .gitignore fix)~
- [ ] think of proper operations for modules:
    - [ ] merge 2 modules (aka resolve 2 modules, and then make sure there are no duplicated paths once all normalized)
    - [x] easy way to create module from a directory (aka use that dir as base, and get all relative paths within it)
    - [ ] interesect 2 modules
    - [ ] NOTE: consider them all need to get the result istantly, since Modules lose abspath information, required to canonicalize!

### long term todos

#### cli ideas

- [ ] add `cleanup` action which acts like untracked FOR ALL MODULES + all possible cleanups,
      such as allow deleting backup dirs without a respective config file, check there are NO
      symlinks in the dotfiles repo, ...
- [ ] add `help` action to get direct help on possible args and flags to the cli
- [ ] add `doc` action to print an entire manual with all things to know about the script
- [ ] add `version`, but no versioning system. NO backward compatibility! version will just be useful for checks on the binary!

#### various

- [ ] have an `--all` flag to specify ALL profiles (or even better: assume it's all, when no specific profile is passed!)
- [ ] allow customizing the `HOME` directory to apply backup to
