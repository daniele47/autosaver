# autosaver

Copy-based dotfiles tracking cli, written in rust

## TEMPORARY NOTES

- eventually, after everything is properly moved into proper docs shipped inside the binary, EVERYTHING
    BUT `universal rules`, `features and bug tracker`, `current items` and `ideas` will be removed.
- If it's something related to using the cli, move it to `docs` subcommand (aka text embedded within the binary!!!)
- If it's meta-explanation, relative to processes like `publishing a new release` or whatnot, then create a markdown in `docs/` directory

## universal rules

1. No `unwraps` in the code, use explicit `assertions to validate invariants`
2. Do `tests` only for important and complex functions that are hard to get right
3. Make sure tests `always cleanup resources` even on panic
4. Make sure there are `no weird debug print` ever left in code, except if meant as part of the cli
5. Add `more comments` to complex functions, just to give general ideas on what is going on
6. Document everything of relevance in `docs/` directory, each within its own markdown file

## features and bug tracker

NOTE: THIS IS JUST AN EXAMPLE, TO DELETE AFTER FIRST STABLE RELEASE, AND REPLACE WITH PROPER THING

### 1.0.0-dev

- FEATURE:
    - added feature 1

- REMOVED:
    - removed feature 2

- BUG:
    - fixed bug 1
    - fixed bug 2

NOTE: - ignore for now, start tracking higher level features added and bugs solved ONLY after the first stable release
      - THIS IS NOT THE TODO LIST. that's just a reminder of what i need to do, on a lower level. and potentially imcomplete.
        This will track ALL features added / removed and bugs fixed. THUS WILL NEED TO BE COMPLETE ALWAYS!

## current items

### 0.1.0-dev

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
- [x] `resolve` func needs to be careful of duplicates by equivalent names (.config/nvim vs .config/nvim/)
- [x] make sure to remove all print from all tests and code! just brutally grep to find them all!
- [x] profile resolver might actually need to be a DFS instead of BFS!
- [x] profile resolver needs to actually add modules to stack too! otherwise breaks!
- [x] make cycle detection more powerful and detect 1 full cycle, for way better error msg! (use `Three-colors DFS`)
- [x] make errors actually be a struct variant (aka {io: ..., path: ...} instead of (ioError,AbsPath)
- [x] move `compile.sh` file into `builds/` itself (tweak logic to make it still work + .gitignore fix)
- [x] modules require only two ops: `resolve` (to turn raw into proper) and `merge_bases` (to sync same module with different bases)
- [x] add filter to `list_files`
- [x] add more filters (symlinks only and dirs only?)
- [x] rename crate to `autosaver`
- [x] rename repo name to `autosaver`
- [x] grep -r to find all `dotfiles / dotfiles-rust` and replace them with `autosaver`
- [x] add the inverse of `read_lines()` to write line by line
- [x] add a test for `read_lines()` and `write_lines()`
- [x] NOTE: `write_lines()` is not symmetrical with `read_lines()`. consider if you want write_lines to take in an iterator of lines instead!
- [x] yup, i indeed do not like the current `write_line()`, which does not have my own return type. think of way to integrate this nicely.
- [ ] add tests for `parsers` (one x each parser type)
- [ ] should avoid splitting profile types, and just have them all in Profile somehow!
- [ ] rework profile:
    - [ ] move list of entries into new struct `Composite`
    - [ ] that way `Profile` now a struct that stores name + profileType (enum with variants contained within)
- [ ] move profile name validity checker to inside `Profile` instead of hardcoded in parser
- [ ] go back to using tuple variants in error type

## long term items

### docs/help

- Flag or subcommand? Current idea: 
    - `--help` flag for just a simplified list of commands and flags (`--help` NEEDS to be appliable to ALL subcommands too, and when cli parsing fails!)
    - `docs` command (with subcommands for each section) for a more throughout explanation of everything.

- Create docs for:
    - General explanations of how the whole dotfiles system works (modules, profiles, backup dirs, ...)
    - Profiles naming limits (only alphanumeric, `-`, `_`)
    - Where dotifles required for this script are placed:
        - `.config/autosaver` for various defaults (like the default profile)
    - Configuration files format (general format rules and all rules specific to profile types: `module`, `profile`, ...)

- Specify profile to apply operation to:
    - allow a command to store said default in `.config` 
        - NOTE: this should probably always prompt, asking if said profile is ok
        - NOTE: this is ALWAYS overriden by all other specified profiles via cli
    - have a way to specify the profile
    - have a flag to specify ALL profiles (or even better: assume it's all, when no specific profile is passed!)

- Subcommands:
    - `save` to apply save action on specified profile
    - `restore` to apply restore action on specified profile
    - `cleanup` action which acts like untracked FOR ALL MODULES + all possible cleanups,
    - `set` to set various configuration option ONLY on the current machine
    - `show` to show all configuration options on the current machine
    - `doc` action to print an entire manual with all things to know about the script
    - `version`, but no versioning system. NO backward compatibility! version will just be useful for checks on the binary!

### various items

- allow customizing the `HOME` directory to apply backup to

- logs for every operation?
    - I could have them shoved into .logs dir, and one file x command run with timestamp so can be easily ordered
    - Or they might just be for dangerous operations, aka probably only for all fs operations
    - I could even log crashes potentially?

- make `autosaver` compatible with all sort of syncing mecchanism
    - do i need to store binary version for any reason? or am i ok with always downloading latest? prolly latest, as that fixes bugs!
    - make the final script store things in $HOME dir, so that the entire dir can be tracked without any issue whatsoever
        - downloaded rust binary: `.cache/autosaver/...`
        - file with defaults to use if nothing else is specified `.config/autosaver/...`
            - NOTE: might be dangerous! Do i want it? if i do, maybe it should show a conferm prompt? Idk, think about it!

## architecture

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

## terminology

- `dotfiles` vs `autosaver`:
    - dotfiles: it refers to the directory that will make use of this autosaver binary to actually track system config files
    - autosaver: it refers to this rust crate and to the rust binary (and to the bash wrapper)
- `profile` vs `module`:
    - profile: (also called composite profile) is a list of profiles or modules
    - module: it is techinically a profile with itself as the only entry, but it also has different config file format
