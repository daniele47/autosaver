# Changelog

All notable changes to this project will be documented here

## v0.11.0

### Features

- `--symlinks|-s` in `clear` to handle broken symlinks too

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
