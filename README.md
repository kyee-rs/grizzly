# Grizzly 🐻

Grizzly is a powerful command-line interface (CLI) tool that allows you to create self-extractable (SFX) archives with
ease.

## Features

- Compress files and folders into a single executable file
- Easily share and extract archives on any computer without additional software
- Customize the extraction process to fit your specific needs

## Installation

To install Grizzly, simply download the latest release from the releases page and add it to your system's PATH.

## Usage

To create an SFX archive, navigate to the directory containing the files and folders you want to compress and run the
following command:

```shell
$ grizzly -f file.txt
```

> Hint: use multiple `-f` tags to compress multiple files/directories, or use `-f .` to compress the current directory
> recursively.

## Help!

```shell
$ grizzly --help
A powerful CLI tool for creating self-extractable (SFX) archives.

Usage: grizzly [OPTIONS] --file <file> [COMMAND]

Options:
  -f, --file <file>          File to compress (use multiple -f flags to compress multiple files).
  -h, --help                 Print help
  -n, --name <name>          Set the name for binary. [Default: Random ID]
  -p, --platform <platform>  Choose the platform for your binary. [possible values: windows/x86_64, windows/x86, windows/aarch64, linux/x86_64, linux/x86, linux/arm, macos/x86_64, macos/aarch64]
  -V, --version              Print version
```