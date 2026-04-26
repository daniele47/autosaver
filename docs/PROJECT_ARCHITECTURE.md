# architecture

- dir structure:

```
dotfiles/
├── autosaver
├── .defaults
├── configs/
│   ├── module1.conf
│   ├── module2.conf
│   ├── profile1.conf
│   └── profile2.conf
└── backups/
    ├── module1/
    └── module2/
```

- module format:

```
/! type module

// policies have always as default
.config/nvim

// after the following line, policy becomes ignore, aka the files reported next will be not tracked,
// even if previous line would have added them, either directly (as files) or indirectly (as part of dirs)

/! policy ignore
.config/nvim/lazy-lock.json 
```

- profile format:

```
/! type profile

neovim
tmux
kde-plasma
```

- autosaver: bash wrapper script to get rust binary (downloaded/compiled) and run it
- .defaults: NOT TRACKED file to store default configurations, things like what module/profile to use by default
- configs: all modules and profile configurations, one config x file
- modules: simple list of files to track
- profiles: groups of modules to apply sequentially
- backups: each module has exactly one backup dir where to save its files, and named like the module

- modules and profiles example formats are reported above
    - // for comments
    - /! for special instruction lines
    - /<char> is extendable in the future, for now ignored!
    - also: spaces should be trimmed, since hopefully no app 
      is insane to use name with starting/ending whitespace

- For example: if `neovim`, `tmux`, `plasma-desktop` are possible modules, `minimal-cli` or `kde-linux` are possible 
  profiles and minimal-cli would only have neovim and tmux, for example

# terminology

- `dotfiles` vs `autosaver`:
    - dotfiles: it refers to the directory that will make use of this autosaver binary to actually track system config files
    - autosaver: it refers to this rust crate and to the rust binary (and to the bash wrapper)
- `profile` vs `module`:
    - profile: (also called composite profile) is a list of profiles or modules
    - module: it is techinically a profile with itself as the only entry, but it also has different config file format
