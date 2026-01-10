# eza-themer

An unofficial eza theme manager that somehow is missing.

Version 0.2.0

It is currently a small tool with only add, switch and list.
But I do not plan to stop here. Upcoming features may include
theme creation aids and better, interactive switch.

Demo:
![Demo](https://github.com/user-attachments/assets/e62d6841-d920-42d5-afab-1bd2355f5138)

# Features

- Easily list, switch, add new themes
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

Proper theme installation will be added in the future,
right now you can `eza-themer add {theme name} {path to theme file}`
or manually copy
