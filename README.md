# markdown-prune

Utility for deleting all markdown files in a folder recursively. Intended to be used as part of a CI/CD pipeline to reduce container size.

## Installation

```sh
curl -fsSL "https://bina.egoist.dev/hougesen/markdown-prune" | sh
```

## Usage

Providing a path to a folder is required.

```sh
# Current path
markdown-prune .

# Absolute path
markdown-prune /path/to/folder

# Relative path
markdown-prune ./node_modules
```

## Disclaimer

Use at your own risk.
