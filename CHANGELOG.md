# Changelog

All notable changes to this project will be documented here

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
