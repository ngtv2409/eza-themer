# Eza Themer

`eza-themer` is a simple interface to manage your eza theme files.
It supports storing multiple themes, easily switch between them and 
apply global overlay.

Version 1.0.0

# Installation

```
cargo install eza-themer
```

# Setup

`eza-themer` finds your themes inside `EZA_THEME_DIR` or 
`XDG_DATA_HOME/eza-themes/` or `HOME/.local/share/eza-themes`.

.themes will not be listed and shouldn't be used. ezt uses them for 
special usage. For example `.overlay.yml` is a special theme which overwrite 
your themes before they are applied.

# Usage

Demo:

![Image](https://github.com/user-attachments/assets/97b43ea4-df44-4c85-ac7d-62c86c80d8b7)

To store your themes, use `ezt add <name> <path>` and it will automatically
copy the file into the correct directory under the correct name.

Name can only contain `a-zA-Z0-9` and `' '`, `'-'`, `'_'`.
It will be transformed into a canonical format, thus allow 
you tonuse many ways to refer to the same theme. The transformations are:
```
s -> to lowercase -> canonically seperate words by '-'
-> strip leading and trailing -
```
Meaning:
```
One dark -> one-dark
one--dark_ -> one-dark
```

You can copy your theme file manually, just make sure that the filename follows the rules above.
