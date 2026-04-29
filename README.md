# autosaver

Copy-based dotfiles tracking cli, written in rust

## universal rules

1. No `unwraps` in the code, use explicit `assertions to validate invariants`
2. Do `tests` only for important and complex functions that are hard to get right
3. Make sure tests `always cleanup resources` even on panic
4. Make sure there are `no weird debug print` ever left in code, except if meant as part of the cli
5. Add `more comments` to complex functions, just to give general ideas on what is going on
6. Document everything of relevance in `docs/` directory, each within its own markdown file

## current items

### 0.1.0-dev

- [x] in fs module, add `list_files` and `all_files` functions
- [x] add proper and comprehensive tests for fs module
- [x] make `list_files` and `all_files` return BTreeSet which is always sorted!
- [x] make error more context aware
- [x] finish implementing `resolve` method in profile.rs
- [x] resolve seems to work, BUT error result is broken (return entire cycle)
- [x] implement tests for `resolve` method in profile.rs
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
- [x] implement modules and profiles `parsers`
- [x] add tests for `parsers` (one x each parser type)
- [x] should avoid splitting profile types, and just have them all in Profile somehow!
- [x] move list of entries into new struct `Composite`, into its own `composite.rs` module in `profile/` dir
- [x] that way `Profile` now a struct that stores name + profileType (enum with variants contained within)
- [x] move `profile.rs`, `composite.rs` (new module to create), `modules.rs` into `profile/` dir
- [x] go back to using tuple variants in error type
- [x] properly split parsers, such that they are all submodules of parsers module, BUT ALL HIDDEN!
- [x] `find_all` file filter does not work (aka it gets applied istantly, and dirs are never even traversed!)
- [ ] add a default implementation for all my traits
- [ ] use `ok_or_else` always, to avoid useless clone operations

## blockers

### 1.0.0

- [ ] full integration tests, not just individual module tests, to test some scenarios
- [ ] complete set of working cli functionality, including `help` command
    - `list` to just list changed files
    - `save` to apply save action on specified profile
    - `restore` to apply restore action on specified profile
    - `cleanup` action which acts like untracked FOR ALL MODULES + all possible cleanups,
    - `help` action to print an entire manual with all things to know about the script
        - (?) `help docs` action to print an entire manual with all things to know about the script
        - (?) each command should get its own help
    - `version`, but no versioning system. NO backward compatibility! version will just be useful for checks on the binary!
- [ ] bash wrapper script to take care of downloading the rust binary and all        
    - (?) keep a timestamp of latest update check? so that i can actually propose to update if current binary is old once in a while
        
## potential future features

- logs for every operation?
    - I could have them shoved into `.cache/autosaver/logs` dir, and one file x command run with timestamp 
        so can be easily ordered + automatic log rotation to keep old latest X log files
    - Idea is that it would be potentially useful to log EVERYTHING!
