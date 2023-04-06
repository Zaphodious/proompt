# Proompt Prompt Program

Generates a prompt!

## Installation

With cargo, run `cargo install --git https://github.com/Zaphodious/proompt.git`,
and reload your terminal.

Then, copy the contents of [the example.sh file](example.sh) into your .bashrc (or equivilant file),
making sure to replace your existing PS2 binding.

# Fonts

Please ensure that a [Nerd Font](https://www.nerdfonts.com/font-downloads)
is being used. To use the default carrot, a [font with extended unicode symbols](https://fonts.google.com/noto/specimen/Noto+Sans+Symbols+2)
should also be installed on your system.

## Usage

The command accepts several arguments. The -i, -c, and -t flags are one-time,
while any number of -s (Section) flags can be passed in and are what make up
the prompt's content.

Colors are accepted in six-digit hex format (eg. ff00aa, f76c59, 07102e),
relying on full-color support from the terminal.

Please always pass the ID of the current 
user via -i, as the program uses this to determine if it is running in a root
shell to improve cross-platform compatability. 

## Example

![Example of trains theme](trains_example.png)

Font is the [Comic Mono Nerd Font](https://github.com/xtevenx/ComicMonoNF)

## Arguments

| Argument | Flag | Parameters | Default | Note |
| --- | --- | --- | --- | --- |
| User ID | -i | number | 1 | Used to detect if root. Please always pass. |
| Prompt Carrot | -c | string | 🮲🮳 | Default requires [font support](https://fonts.google.com/noto/specimen/Noto+Sans+Symbols+2) |
| Theme | -t | string | trains | Currently, only "trains" exists |
| Section | -s | background-color foreground-color string | None | Displays the string as a section using the indicated colors |

## Future

- Git support
- More themes 
- ???
- Profit...?

