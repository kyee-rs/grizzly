# Grizzly 🐻 - Self-extractable (SFX) archives creator.

### Philosophy

I've created `Grizzly` as a hobby project. I thought that we don't need
long commands (`tar -xvf ...`) or external apps to create and unzip an archive anymore.
Create binaries that store all your files inside with only `453 KB` overhead and distribute them wherever you want.

### CLI

```shell
$ grizzly --help
SFX (Self-extractable) archives generator.

Usage: grizzly [OPTIONS] --file <file> [COMMAND]

Commands:
  prepare  Download the Go compiler to bundle an archive.
  help     Print this message or the help of the given subcommand(s)

Options:
  -f, --file <file>          File to bundle (use multiple -f flags to bundle multiple files).
  -p, --platform <platform>  Platform to build your bundle for. [possible values: linux/386, linux/amd64, linux/arm, linux/arm64, windows/386, windows/amd64, windows/arm, windows/arm64, darwin/386, darwin/amd64, darwin/arm, darwin/arm64]
  -n, --name <name>          Set the name for binary. [Default: Random ID]
  -h, --help                 Print help
  -V, --version              Print version
```

**Usage:**

```shell
$ grizzly -f file.txt
```