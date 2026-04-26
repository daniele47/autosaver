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
- [x] ~modules require only two ops: `resolve` (to turn raw into proper) and `merge_bases` (to sync same module with different bases)~
- [x] ~add filter to `list_files`~
- [x] ~add more filters (symlinks only and dirs only?)~
- [x] ~rename crate to `autosaver`~
- [ ] rename repo name to `autosaver` (both on github and codeberg!)
- [x] ~grep -r to find all `dotfiles / dotfiles-rust` and replace them with `autosaver`~
- [ ] add the inverse of `read_lines()` to write line by line

## long term items

### cli items

- [ ] add `cleanup` action which acts like untracked FOR ALL MODULES + all possible cleanups,
      such as allow deleting backup dirs without a respective config file, check there are NO
      symlinks in the dotfiles repo, ...
- [ ] add `help` action to get direct help on possible args and flags to the cli
- [ ] add `doc` action to print an entire manual with all things to know about the script
- [ ] add `version`, but no versioning system. NO backward compatibility! version will just be useful for checks on the binary!

### various items

- [ ] have an `--all` flag to specify ALL profiles (or even better: assume it's all, when no specific profile is passed!)
- [ ] allow customizing the `HOME` directory to apply backup to

### ideas

- logs for every operation?
    - I could have them shoved into .logs dir, and one file x command run with timestamp so can be easily ordered
    - Or they might just be for dangerous operations, aka probably only for all fs operations
    - I could even log crashes potentially?

- make the final script store things in $HOME dir, so that the entire dir can be tracked without any issue whatsoever
    - downloaded rust binary: `.cache/autosaver/...`
    - file with defaults to use if nothing else is specified `.config/autosaver/...`
        - NOTE: might be dangerous! Do i want it? if i do, maybe it should show a conferm prompt? Idk, think about it!
