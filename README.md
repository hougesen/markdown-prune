# markdown-prune

Utility for deleting all markdown files in a folder recursively. Intended to be used as part of a CI/CD pipeline to reduce container size.

## Installation

```sh
curl -fsSL "https://bina.egoist.dev/hougesen/markdown-prune" | sh
```

## Usage

### Basic usage

Providing a path to a folder is required.

```sh
# Current path
markdown-prune .

# Absolute path
markdown-prune /path/to/folder

# Relative path
markdown-prune ./node_modules
```

### Explicit arguments

#### Path

`--path` is used to pass an explicit path.

```sh
markdown-prune --path ./node_modules
# Deleted 2185 files totaling 12.03mb
```

#### Dry running

If you want to dry run first before deleting the files, either pass `--dry` or `--check` as an argument. Dry running only works with an explicit path argument.

```sh
markdown-prune --path ./node_modules --check
# Found 2185 files totaling 12.03mb
```

## Disclaimer

Use at your own risk.
