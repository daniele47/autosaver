# autosaver

Copy-based dotfiles tracking cli, written in rust

## How to install and use

```sh
curl -fsSL https://raw.githubusercontent.com/daniele47/autosaver/refs/heads/main/install.sh | bash -s
```

or manually download a binary from the [github repository](https://github.com/daniele47/autosaver/releases)

## How it works

This is a rust based cli program to easily handle dotfiles on a system.

The entire system is built with one basic concept: profiles. Everything is a profile!

Profiles can be created by adding a new file in the `config` directory, with the `.conf` extension.
Such profiles will be automatically loaded as their path relative to the `config` directory, stripped
of the `.conf` extension.
So `config/neovim.conf` will be loaded as `neovim` profile.

Profiles, also, don't need to be files directly in the `config` directory. They can also be nested!
So `config/cli/tmux.conf` will be loaded as the `cli/tmux` profile.

## Configuration files

All profiles share some basic properties:
- all lines starting with `//` are `comments` and completely ignored
- all lines starting with `/!` are `option lines`, and can be used to change various settings
- all other lines starting with `/` are reserved for future use, thus currently ignored
- all other lines are `data lines`, aka the actual entries of the profile itself

NOTE:
- all profiles share the `kind` and `id` options:
    - `type` is required, and specifies the profile type
    - `id` is optional and is used to locate the resources associated with the profile

### Composite Profile

This profile simply acts as a profile aggregrator, and can be used to run command on multiple profiles.
It can be created via `type composite` option line, or by creating a directory which will be automagically
treated as a composite profile aggregrating the files and directories directly in it.

- `option lines`: no additional option lines!

- `data lines`: each data line is exactly the name of an other profile

### Module Profile

This profile is the one actually allowing to track dotfiles on the system.

<!-- TODO: REWRITE AFTER -->

This is a profile to track dotfiles. It can list file paths relative to the $AUTOSAVER_HOME directory, and it is 
used in `list`, `save`, `restore`, `rmhome`, `rhbackup` commands to confront the files on the home and in the 
backup directory, and if the two differs, they can be updated based on the command specified.

Module entries can have a `policy` with priority matching the following order:
- `ignore`: force ignore the specified path, overriding all other policies
- `notdiff`: do not show the path in commands if it differs between the two versions
- `always` \[DEFAULT\]: always show the path, if the two versions do not match

A Module profile configuration file looks like this:
```
/! type module
/! id neovim

// this is a comment
// NOTE: directories do not require an ending slash!

.config/nvim

/! policy notdiff

.config/htop

/! policy ignore

.config/nvim/lazy-lock.json
```
This profile will:
- use `backup/neovim` as the backup directory
- track all files found recursively in `$AUTOSAVER_HOME/.config/nvim/` and in `$AUTOSAVER_ROOT/backup/.config/nvim` files with `always` policy
- track all files found recursively in `$AUTOSAVER_HOME/.config/htop/` and in `$AUTOSAVER_ROOT/backup/.config/htop` files with `notdiff` policy
- ignore `.config/nvim/lazy-lock.json` file that was included with the first line (`ignore` policy)

### Runner

This is a profile to list scripts from the `run` directory, that will be executed in order with the `run` command.

Runner entries can have a `policy` with priority matching the following order:
- `skip`: do not run the scripts at the specified path
- `run`: \[DEFAULT\]: run the scripts at the specified path

A runner profile configuration file looks like this:
```
/! type runner
/! id kde-init

// this is a comment
// NOTE: directories do not require an ending slash!

init_script.sh
kde-init/

/! policy skip

kde-init/data

```
This profile will:
- use the `run/kde-init` as the run directory (as specified by `id`)
- run the `init_script.sh` script
- run all the files found in `kde-init/` path
- will skip all scripts in `kde-init/data` path

NOTES: 
- scripts DO NOT ALLOW STDIN! This is intentional, as interactive init scripts are a bad idea!
- an hacky workaround is to allow environment variables to customize init behaviour, or in cases, such 
    as getting root permissions, just do it with a wrapper bash script that keeps it cached!

