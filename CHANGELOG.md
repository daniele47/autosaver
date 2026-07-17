# Changelog

All notable changes to this project will be documented here

## v2.13.0

### Features

- [WIP] added `--allow-cleanup|-c` which prompts to delete files/dir specified in profile config file
    -> [WIP] this allows profile configs to specify directories handled by the program relative to the profile, and easily clean it up
    -> [WIP] NOTE: cleanup is prompted BEFORE prompting for all else. even prior calculating files presence on disk!
- added `/! cleanup <PATH>` option line for module profiles, tied to --allow-cleanup flag
    -> [WIP] update module parse test with this newer option

### Changes

- stricter config parser rejects all lines starting with `/` but not `//`

## v2.12.7

### Changes

- made `policy` option uniform between profiles

## v2.12.6

### Changes

- removed `-p` flag allowing multiple profiles (too weird to use)

## v2.12.5

### Changes

- `-l` ---> `-N` (made no sense to have -l for --dry-run)

## v2.12.4

### Patches

- `--no-color|-C` didn't previously remove coloring at all (only the env var `NO_COLOR` worked previously)

## v2.12.3

### Changes

- reorganized cli flags, to heavily reduce global flags, and replace them with command relative flags

## v2.12.2

### Changes

- made `-D` flag only check for relative path duplicates

## v2.12.1

### Changes

- compacted `-p` and `-P` into a single `-p` flag

## v2.12.0

### Features

- added `--no-color|-C` flag (`NO_COLOR` env var) to disable colors entirely (accessibility flag)

### Changes

- reorganized help message, to include a `Global Options` section, for easier parsing

## v2.11.2

### Changes

- added nice separator between different prompts, making it easier on the eye to parse prompts
- now diff output has no context, showing only exactly what changed
- add `default` color for everything that by default doesn't get colored
- add `whitespaces` color for all whitespace between elements of different types

## v2.11.1

### Patches

- safer and more robust install script and command

## v2.11.0

### Features

- added `--auto-answer|-A` flag to allow listing answers to automatially give to all prompts

## v2.10.1

### Patches

- fully removed `PERF` environment variable, which was too hacky and garbage overall

## v2.10.0

### Features

- added `--profiles|-P` that allows specifying 0..N profiles, and runs them in orders

## v2.9.1

### Patches

- reserved profile names should be instantly checked for!
- use `-D` for `--allow-duplicates` to avoid conflicts

## v2.9.0

### Features

- added `--allow-duplicates|-d` to avoid early exiting when duplicated paths are found

## v2.8.5

### Patches

- invalid input error now trims input string, for better output

## v2.8.4

### Changes

- changed colors: unmodified now use default path color, and there is proper distinction between missing files on creating/deleting side

## v2.8.3

### Patches

- unique path checks now canonicalize path prior to checking, since otherwise symlinks might end up getting the file rewritten multiple times

## v2.8.2

### Patches

- warnings for missing flags are now slightly more significative where necessary

## v2.8.1

### Patches

- added `-s|--symlink` flag now actually works

## v2.8.0

### Features

- added `-s|--symlink` flag to allow deleting symlinks

### Patches

- added warning when `purge_path` is unable to delete path

## v2.7.1

### Patches

- `PERF=` removed from accepted default env vars, as it cannot be loaded fast enough

## v2.7.0

### Features

- added `.autosaver/colors` file to allows specifying a different colorscheme from the default one

## v2.6.0

### Features

- added `.autosaver` directory as a `ROOT` directory top level marker + to store machine dependent configs
- added `.autosaver/env` file to allow specifying defaults for ALL env vars the program considers

### Patches

- missing `config` directory is treated as empty directory, no langer it reports an error

## v2.5.2

### Patches

- `purge_path` is now safer, as it NEVER deletes directories (other then empty parent directories of files)

## v2.5.1

### Patches

- added warning for files that need `-f` to be deleted
- delete action now prints path ONLY if there is something to be deleted
- `clear` action now also is able to find and prompt to delete ignored files that somehow got in

## v2.5.0

- add flag `-c|--choice` that prompts to even allow executing profiles
- `-c|--choice` is also able to edit/show config file relative to the profile

### Patches

- handled choice in prompt now removes `f` answer when no paths are specified

## v2.4.4

### Patches

- now `list` not prompts for unmodified files

## v2.4.3

### Changes

- sorted virtual composite profiles (aka directories) entries in alphabetical order

## v2.4.2

### Changes

- easier to read `PERF=` output

## v2.4.1

### Patches

- fixed `all` profile not loading correctly
- disabled configurations starting with `.` as that often breaks
- changed `type` to `kind` in config files
- improved error messages

## v2.4.0

### Features

- added `-I|--ignore` flag for `tree` command to ignore profiles recursively

### Optimization

- configs load almost twice as fast, by avoiding querying for files inside directories twice

## v2.3.0

### Features

- added `f` prompt answer that simply prints all full paths involved in the prompt

### Changes

- added prompts for unmodified files too (such that actions can be performed on them too!)
- removed env variables section in help msg that felt out of place
- made clicolor store functions as methods instead of functions

## v2.2.4

### Changes

- path uniqueness check now is skipped entirely in `list` action

## v2.2.3

### Changes

- fixed custom `Environment` variables section in help message

## v2.2.2

### Changes

- improved `PERF` performance output to be nicer and more comprehensive
- documented various useful environment variables hackily using clap

## v2.2.1

### Changes

- `-l|--list` flag now also skips all checks, such as path uniqueness

## v2.2.0

### Features

- added `PERF` env variable, to see various performance times

## v2.1.0

### Features

- implemented `clear` action

### Patches

- properly integrated `warning` messages into prompt
- removed newline from prompt, to make them more consistent
- make `clear` and `backup` function properly
- fixed `ctrl+d` breaking nice output (just wrapped all `inputln!` calls to print newline if input not end with one itself)

### Changes

- split colors from context, in view of future color customization
- infrastructure build to dynamically load theme

## v2.0.1

### Patches

- fixed crash when using `list` command

## v2.0.0

### Features

- complete rewrite, using 3rd party crates as dependencies
- added `--log` flag with proper logs
- added `all` as the default root profile tracking ALL profiles
- added `--home|--root` flags to specify different root and home directories (defaults: $HOME, current dir)
- added `--list|-l` to not even show prompts (basically it's -n BUT nicer)
- added very powerful prompt handler
- added `--stdin|-i` flag for `run` command to allow stdin when running init scripts
- added option in `runner` profiles: `/! stdin` to hint tht
- added `--dangerous|-d` flag in `save|restore` to handle deleting files

### Changes

- replaced `.default` loaded by rust, with a `.env` script loaded from the bash wrapper
- `--profile` defaults to `all` if missing!
- removed bash wrapper script, replaced with a single install script

### Patches

- add uniqueness checks for `type` and `id` lines in config files (as those make no sense to have repeated)

## v0.18.1 / v1.0.0

### Changes

- multithreaded stdout/stderr handler, which nicer split between the two visually

## v0.18.0

### Features

- added `--show-types|-t` for `list|save|restore` commands

## v0.17.5

### Changes

- brough back support for symlinks in dir (just with check they don't escape containment)

## v0.17.4

### Patches

- fixed inout to make sure there are no broken colors

## v0.17.3

### Patches

- cut output now stays precisely in 80 char width

## v0.17.2

### Patches

- `clear` now checks ALL, to allow directories shared between profiles

## v0.17.1

### Patches

- disabled hidden config files and directories, as those make possible to override . and "" (bad!)!

## v0.17.0

### Features

- added `--add|-a` to `clear` command, to allow deleting untracked files outside profile dirs

## v0.16.1

### Patches

- fixed debug msg for symlink checks

## v0.16.0

### Features

- added `dir` option type in `module|runner` profiles to indicate the dirname (allows easy config refactor)
- added `--show-dir|-d` flag to show the directory in `tree` command

## v0.15.1

### Patches

- deleting symlinks now delete all parent dirs too

## v0.15.0

### Features

- added `--unique|-u` to `tree` command to skip already seen profiles

## v0.14.4

### Patches

- fixed wrongly parsed single letter flag 

## v0.14.3

### Patches

- made backtrace capture optional, as it significantly slows down the program on failure

## v0.14.2

### Patches

- errors and warnings now don't color the entire line anymore, just the `ERROR|WARNING:` part 

## v0.14.1

### Patches

- do not panic on broken stdout/stderr. Just keep running silently

## v0.14.0

### Features

- added single letter flag shortscuts to tree word flags
- added `--ascii|-a` flag for `tree` command to use only ascii characters

### Patches

- added checks for profile used, to avoid parent dirs in it and to avoid it being an absolute path
- improved err msg for commands with invalid args

## v0.13.0

### Features

- added `tree` command to display the resolution tree of profiles
- added `--short-names` flag for `tree` command to show only basename of profiles
- added `--show-types` flag for `tree` command to show the type of the profiles

### Patches

- stricter `Runner` methods, now don't borrow mutably anymore

## v0.12.1

### Changes

- file diffs now separate different diff blocks with a nice `@` sign

## v0.12.0

### Features

- added `backtrace` for errors
- adding debug options to `inout`
- added `--debug` flag to show debug output
- added more logs for `--debug`

## v0.11.0

### Features

- `--symlinks|-s` in `clear` to handle broken symlinks too
- `--unmodified|-u` flag in `list` command to show also tracked but not modified files

### Patches

- fixed broken symlinks causing an error. Now they are handled properly
- fixed `list` command not showing deleted files

## v0.10.1

### Changes

- now `clear` command accepts a profile, and only clears relative to that profile

## v0.10.0

### Features

- added `clear` command to remove all untracked files from `run` and `backup` dirs

### Changes

- added back `list` as semplified alias for `save -l`

## v0.9.1

### Fixes

- use 80 as line len everywhere

## v0.9.0

### Features

- `--full|-f` flag to show entire diff, script and script output

## v0.8.5

### Changes

- changed `list` command with `--list|-l` flag

## v0.8.4

### Changes

- allow missing `run`, `backup`, `config` directories
- made flags consistent, by actually splitting words with a `-` 

## v0.8.3

### Patches

- symlinks check doesn't run on help/version actions

## v0.8.2

### Changes

- added `$` to start of prompt lines

## v0.8.1

### Patches

- updated colors for rmhome and rmbackup paths
- parser does not allow .. paths anymore
- binary now checks no symlink that links to outside the repo exist before running 

## v0.8.0

### Features

- config directories are now treated exactly as if they were composite profiles loading the files within

## v0.7.4

### Changes

- added `ls` alias for `list`

## v0.7.3

- added strict command checks

## v0.7.2

### Patches

- fixed `autosaver` script deleting old version even if newer fails to install
- actually parse stdout and format it nicely

## v0.7.1

### Fixes

- proper color for line separator between scripts

## v0.7.0

### Features

- bash `autosaver` script allows specifying the precise version to download

## v0.6.4

- run command now isolates scripts based on the profile

## v0.6.3

### Changes

- `--dryrun` flag turned into `-l|--list` flags in `run` command

## v0.6.2

### Patches

- script output now isn't parsed, and kept as is
- script output now it's ended by a clear line separator
- added output showing the main profile before all else

## v0.6.1

### Patches

- add `.default` file to help msg
- fixed global help msg invalid line

## v0.6.0

### Features

- added `.default` configuration file to specify a default profile

### Patches

- updated help msg to explain global flag better
- make scripts automatically executable before running them

## v0.5.1

### Patches

- fixed error printing one extra whiteline
- better err msg for when no profile is specified
- captured scripts output and formatted nicely
- very simple multithreaded stdout/stderr

## v0.5.0

### Features

- added new runner profile
- added new run command

### Patches

- removed valid name checker for composite profiles
- fixed `-n|--assumeno` not actually properly working
- fixed `-n|--assumeno` and `-y|--assumeyes` output, now they print y|n properly

## v0.4.2

### Patches

- updated bash script to shorten its runtime, and get a faster running cli
- fixed `--assumeyes|-y` and `--assumeno|-y` flags, now properly skipping prompt

## v0.4.1

### Patches

- flags with no commands now are treated as errors too.
- better profile output color
- binary files are now properly handled by the program when diffing
- fixed `content_eq` function, and now is able to compare binary files

## v0.4.0

### Features

- implemented Myers algorithm to diff files, and used in new `-d|--diff` flag in `save|restore|list`
- added q into prompt to instantly quit

### Patches

- improved cli colors

## v0.3.5

### Removals

- removed autocompletions, and self-updating bash script

## v0.3.4

### Patches

- install script now shows downloaded version

## v0.3.3

### Patches

- changing bash script to have almost no logic, and leave all the logic in update scripts in the repo itself
- update help msg to list environment variables

## v0.3.2

### Patches

- fixes `-a` not working with list command

## v0.3.1

### Patches

- fixed `rmhome|rmbackup` not showing the paths

## v0.3.0

### Features

- added `AUTOSAVER_HOME` and `AUTOSAVER_PROFILE` configuration env variables
- added `rmhome` and `rmbackup` actions, to delete files from home/backup directories
- implemented `--help` to get help messages from current available commands

### Patches

- always show relative paths instead of home paths (which was very arbitrary and useless)

## v0.2.0

### Features

- added bash script to automatically download latest autosaver binary

## v0.1.0

### Features

- added `list`, `save`, `restore` commands to list differences between home and backup, and save/restore them
- `--version` to get the binary current version
