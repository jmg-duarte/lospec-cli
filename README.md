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

### Cargo

To install Lospec CLI using `cargo`, run the following command:

```
cargo install lospec-cli
```

### Source

To install Lospec CLI from source, follow these steps:

1. Clone this repository:

```
git clone https://github.com/lospec/lospec-cli.git
```

2. Navigate to the project directory:

```
cd lospec-cli
```

3. Install:

```
cargo install --path .
```


## Examples

### Searching for Color Palettes

To search for color palettes, use the `search` command followed by your query:

```
lospec search --tag purple --max 8 -p 2 --sorting az
```

In other words:
* `--tag "purple"` — search for the color schemes with a "purple" tag
* `--max 8` — search for a palette with a maximum of 8 colors
* `-p 2` — show the second page of results
* `--sorting az` — sort results alphabetically


### Downloading Color Palettes

To download a color palette, use the `download` command followed by the palette's slug:

```
lospec download slso8 -f colorset
```

In other words:
* `download sls08` — download the `sls08` palette
* `-f colorset` — save the palette in XCode's `.colorset` format


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

