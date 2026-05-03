# Changelog

All notable changes to this project will be documented here

## WIP

- [ ] --assumeyes and --assumeno flags (and their letter flag versions) stopped working entirely

### Patches

- updated bash script to shorten its runtime, and get a faster running cli

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
