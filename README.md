# autosaver

Copy-based dotfiles tracking cli, written in rust

## How it works

```
$AUTOSAVER_ROOT
├── backup
│   └── profile1
└── config
    └── profile1.conf
```

It is a rust based cli program to easily handle dotfiles on a system.

The entire system is built with one basic concept: profiles. Everything is a profile!

To create a new profile, just create a `config/<profile_name>.conf` file and then the 
cli will automatically detect that as the configuration for the `<profile_name>` profile.

## Profiles

### Composite 

This is a profile that list other profiles. It can list ALL types of profiles, even other
composite profiles, and it will make commands run on the profiles listed. 

There are no limitations on including composite profiles in other composite profiles, but
there are the following rules:
- profiles are resolved in the order they are listed in the config file
- duplicated are ignored, and only the first entry of duplicates is ever considered
- if composite profile include composite profile that include themselves in any way (aka,
    any sort of cyclic dependency is create), the cli will detect it and quit with an error

A composite profile configuration file looks like this:

```
/! type composite

// this is a comment

profile1
profile2
```
This profile will contain the profile1 and profile2, and when any action is run on it, it will
actually run on `profile1` and `profile2`

### Module

This is a profile to track dotfiles. It can list file paths relative to the $AUTOSAVER_HOME directory,
and it is used in `list`, `save`, `restore` commands to confront the files on the home and in the 
backup directory, and if the two differs, they can be updated based on the command specified.

Module entries can have a `policy` with priority matching the following order:
- `ignore`: force ignore the specified file, overriding all other policies
- `notdiff`: do not show the file in commands if it differs between the two versions
- `always` \[DEFAULT\]: always show the file, if the two versions do not match

A Module profile configuration file looks like this:
```
/! type module

// this is a comment

.config/nvim

/! policy notdiff

.config/htop

/! policy ignore

.config/nvim/lazy-lock.json
```
This profile will:
- track all files found recursively in `$AUTOSAVER_HOME/.config/nvim/` and in `$AUTOSAVER_ROOT/backup/.config/nvim` files with `always` policy
- track all files found recursively in `$AUTOSAVER_HOME/.config/htop/` and in `$AUTOSAVER_ROOT/backup/.config/htop` files with `notdiff` policy
- ignore `.config/nvim/lazy-lock.json` file that was included with the first line (`ignore` policy)

## How to use

- just copy `scripts/autosaver` bash script in the directory you want to use to save dotfiles in

The first time the script is run, it will download the rust autocompiled binary by and hosted on `github`, and it will
store it in the `cache` directory, properly following `xdg-base` specifications. 

NOTE:
- run the bash script to install the rust binary and completions if missing
- run the bash script with `INSTALL= ./autosaver` to force an update of binary and completions
- run the bash script with `UNINSTALL= ./autosaver` to remove all things installed by the program 

NOTE: Just run `./autosaver --help` to get list of commands, flags and environment variables
