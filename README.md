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

NOTES for future reference:
    - `HOME`: is the user home directory, unless specified otherwise
    - `ROOT`: is the dotfiles directory, in which `config`, `backup`, `run` directories are located 

## Configuration files

All profiles share some basic properties:
- all lines starting with `//` are `comments` and completely ignored
- all lines starting with `/!` are `option lines`, and can be used to change various settings
- all other lines starting with `/` are reserved for future use, thus currently ignored
- all other lines are `data lines`, aka the actual entries of the profile itself

### Composite Profile

This profile simply acts as a profile aggregrator, and can be used to run command on multiple profiles.
It can be created via `type composite` option line, or by creating a directory which will be automagically
treated as a composite profile aggregrating the files and directories directly in it.

- `option lines`:
    - `kind`: must be `composite`

- `data lines`: 
    - each data line is exactly the name of an other profile

### Module Profile

This profile is the one actually allowing to track dotfiles on the system. 
This profile is just a list of paths, each with a priority. Note that directories actually resolve
to all the file in them instead, thus all operations happen on files only.
The saved files will end up in the `backup/<profile_id>` directory

- `option lines`:
    - `kind`   : must be `module`
    - `id`     : identifies the backup directory
    - `policy` : specify the policy for all the files after it
        - `ignore`  : always ignore path
        - `notdiff` : do not show by default if `HOME` and `ROOT` version just differ
        - `always`  : always show path

- `data lines`: 
    - each data line is a relative path of the dotfiles to track in the `HOME`

### Module Runner

This profile is to save and easily run various init or any other kind of script.
The scripts are search from the `run/<profile_id>` directory

- `option lines`:
    - `kind`   : must be `runner`
    - `id`     : identifies the run directory
    - `policy` : specify the policy for all the files after it
        - `skip` : ignore the script
        - `run`  : run the script
    - `stdin` `on|off` : hints about enabling stdin

- `data lines`: 
    - each data line is a relative path of the script to track

## Environment variables not specified in help text

- `PERF`  : prints the performance of various sections of the code
- `EDITOR`: to pick the editor used with `e` answer in prompts
