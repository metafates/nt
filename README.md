# nt

Minimal  cli snippet manager
inspired by [boom](https://github.com/holman/boom)

## Usage

```
USAGE:
nt <SUBCOMMAND>

OPTIONS:
-h, --help       Print help information
-V, --version    Print version information

SUBCOMMANDS:
get       get record
help      Print this message or the help of the given subcommand(s)
list      list all records
remove    remove a record
set       set a new record
```


## Example

```bash
$ nt set magic 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'
'magic' set to 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'

# if not value is given, the value is read from the clipboard
$ nt set num
'num' set to contents from clipboard

$ nt get num
1

$ nt get num --copy
copied to clipboard

$ nt remove num # or `nt rm`
'num' removed

$ nt list # or `nt ls`
'magic' - 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'

'greeting' - 'hello!'
```

> Data is stored in the default config directory under `nt` folder.