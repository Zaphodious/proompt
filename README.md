# Proompt Prompt Program

Generates a prompt!

## Installation

With cargo, run `cargo install --git https://github.com/Zaphodious/proompt.git`,
and reload your terminal.

Then, copy the contents of [example.sh] into your .bashrc (or equivilant file),
making sure to replace your existing PS2 binding.

## Usage

The command accepts several arguments. The -i, -c, and -t flags are one-time,
while any number of -s (Section) flags can be passed in and are what make up
the prompt's content.

Please always pass the ID of the current 
user via -i, as the program uses this to determine if it is running in a root
shell to improve cross-platform compatability. 


## Arguments

| Argument | Parameters | Default | Note |
| --- | --- | --- | --- |
| -i | number | 1 | ID of the user. Used to detect if root. Please always pass. |
| -c | string | 🮲 🮳 | The prompt carrot (default requires [font support](https://fonts.google.com/noto/specimen/Noto+Sans+Symbols?query=noto+sans+symbols)) |
| -t | string | trains | The name of the theme. Currently, only "trains" exists |
| -s | None | background-color foreground-color string | Displays the string using the indicated colors |

## Future

- Git support
- More themes 
- ???
- Profit...?

