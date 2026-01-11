# eza-themer

An unofficial eza theme manager which should not be missing

Version 0.3.0

Demo:

![Demo](https://github.com/user-attachments/assets/394d6924-c834-4a12-b018-8582e38b0569)

# Features

- Easily list, switch, add new themes
- Globally overlay all of your themes
- Basic interactivity

> [!NOTE]
> It works by copying the theme file into your eza config
> directory. Be aware that it will OVERRIDE your old themes.

# Installation

You can easily get it from source or cargo
```
cargo install eza-themer
```

# Setup

`eza-themer` find your themes inside `EZA_THEME_DIR` 
or `XDG_DATA_HOME/eza-themes/` or `HOME/.local/share/eza-themes`

You can put your themes there as `${theme_name}.yml`s.

`.themes` will not be listed and shouldn't be used. `ezt` uses them 
for special usage. For example `.overlay.yml` is a special theme 
which overwrite your themes before they are applied.

Proper theme installation will be added in the future,
right now you can `eza-themer add {theme name} {path to theme file}`
or manually copy them to the designated directory.
