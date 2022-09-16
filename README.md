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

#### Custom filenames & extensions

If you wish to remove files you can use the following commands `--file`, `--file-name` or `--ext`.

```sh

markdown-prune --path ./node_modules --file .DS_Store --file .d.ts

# Found 25216 files totaling 114.10MB
```

#### Result unit size

The argument `--unit` can be used to change the default result byte size.

It accepts the following `b`, `kb`, `mb`, `gb`, `tb`

```sh
markdown-prune --path ./node_modules --check --unit b
# Found 2185 files totaling 12030000b

markdown-prune --path ./node_modules --check --unit kb
# Found 2185 files totaling 12030kb

markdown-prune --path ./node_modules --check --unit mb
# Found 2185 files totaling 12.03mb

markdown-prune --path ./node_modules --check --unit gb
# Found 2185 files totaling 0.01gb

markdown-prune --path ./node_modules --check --unit tb
# Found 2185 files totaling 0tb
```

## Disclaimer

Use at your own risk.
