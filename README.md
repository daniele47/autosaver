# autosaver

Copy-based dotfiles tracking cli, written in rust

## How to install and use

```sh
tmp=$(mktemp) &&
    curl -fsSL "https://raw.githubusercontent.com/daniele47/autosaver/refs/heads/main/install.sh" -o "$tmp" &&
    printf '%s  %s\n' "7533b3630efdc23b20ef372f908eb7f198b0f8785e2f4659856056abb68d560c" "$tmp" | sha256sum -c &&
    bash "$tmp" ||
    echo -e '\e[1;31merror:\e[m Installation failed!\e[m'
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

## Machine configuration directory

- a `.autosaver` marker directory MUST BE created in the `ROOT` directory. It has various uses:
    1. it acts as a top level dir marker. Basically it specifies that a directory is a autosaver repository.
    2. it allows specifying machine dependent configurations:
        - `.autosaver/env` to specify the default values for ALL the environment variables of the program
            - commented lines (`# commented line`): are lines starting with `#`, which are just ignored
            - env var lines (`ENV=value`): are key-value lines, with a `=` in the middle splitting the two
            - accepted env var: `AUTOSAVER_HOME`, `AUTOSAVER_PROFILE`, `EDITOR`
            - note: all whitespace is insignificant, and always trimmed!
        - `.autosaver/colors` to specify different colorscheme for various output elements
            - commented lines (`# commented line`): are lines starting with `#`, which are just ignored
            - colors lines are a serie of words: `<ELEMENT> [<STYLE>...]`
                - `ELEMENT`: specifies what element to apply the styles to
                - `STYLE`: specify what style to apply to the element

NOTE: this directory SHOULD not be tracked with git or other sync mecchanisms to save and share the repository

### Colorscheme configuration file

```
# vim: set commentstring=#\ %s:

# color used for all not colored text
default

# color of `-` symbol before deleted lines in diff outputs
diff_deleted red
# color of `@@@` headers in diff outputs
diff_header cyan
# color of `+` symbol before deleted lines in diff outputs
diff_inserted green

# color of files in backup action when they differ
output_diff bright_yellow
# color of files in save and restore actions when file is to be created
output_create green
# color of files in save and restore actions when file is to be deleted
output_delete red
# color of files in list action when one file is missing
output_missing red
# color of file paths with nothing specific about them
output_path bright_blue
# color of profiles names
output_profile purple

# color of the [a/b/c/...] choices in prompts
prompt_choices bright_black
# color of the message in prompts
prompt_msg italic bright_black underline

# color of `@@` header in show prompt action
show_header cyan

# color of composite profiles in tree action
tree_composite
# color of (*) symbol that denotes duplicated profiles in tree action
tree_dedup yellow
# color of module profiles in tree action
tree_module bright_blue
# color of runner profiles in tree action
tree_runner green

# foreground colors
# black red green yellow blue magenta purple cyan white
# bright_black bright_red bright_green bright_yellow bright_blue
# bright_magenta bright_purple bright_cyan bright_white
#
# background colors
# on_black on_red on_green on_yellow on_blue on_magenta on_purple on_cyan on_white
# on_bright_black on_bright_red on_bright_green on_bright_yellow on_bright_blue
# on_bright_magenta on_bright_purple on_bright_cyan on_bright_white
#
# modifiers
# bold dimmed italic underline blink blink_fast reversed hidden strikethrough
```

## Configuration files

All profiles share some basic properties:
- all lines starting with `//` are `comments` and completely ignored
- all lines starting with `/!` are `option lines`, and can be used to change various settings
- all other lines starting with `/` are reserved for future use, thus currently ignored
- all other lines are `data lines`, aka the actual entries of the profile itself

### Composite Profile

This profile simply acts as a profile aggregrator, and can be used to run command on multiple profiles.
It can be created via `kind composite` option line, or by creating a directory which will be automagically
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

## Extra notes

### Environment variables not specified in help text

- `EDITOR`: to pick the editor used with `e` answer in prompts

### Reserved profile names

- `all`: top profile, used when none are specified, and from which ALL profiles can be found
- `custom`: virtual profile that contains the profiles specified by `--profiles|-P` flag
