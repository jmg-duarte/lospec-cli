# Lospec CLI

Lospec CLI is a command-line interface tool that allows users to interact with Lospec's color palettes conveniently. With this tool, you can search for color palettes and download them.


## Usage

```
lospec <COMMAND>
```

### Commands

- `search`: Search for color palettes
- `download`: Download a color palette
- `help`: Print this message or the help of the given subcommand(s)

### Options


- `-h, --help`: Print help

## Installation

To install Lospec CLI, follow these steps:

1. Clone this repository: `git clone https://github.com/lospec/lospec-cli.git`
2. Navigate to the project directory: `cd lospec-cli`
3. Install: `cargo install`


## Examples

### Searching for Color Palettes

To search for color palettes, use the `search` command followed by your query:

```
# Search for
# * containing the tag "purple"
# * a palette with a maximum of 8 colors
# * show the second page
# * sort results alphabetically
lospec search --tag purple --max 8 -p 2 --sorting az
```

### Downloading Color Palettes

To download a color palette, use the `download` command followed by the palette's slug:

```
# Download the `sls08` palette and save as XCode's `.colorset`
lospec download slso8 -f colorset
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
