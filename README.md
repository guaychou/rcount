# RCount 

### RCount will help you to count a number of file hidden or not in a directory, and written in rust

#### How to use

```
RCount tool 0.1.0

USAGE:
    rcount [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Recursive count or not
    -V, --version      Prints version information

OPTIONS:
    -d, --dir <dir>    Directory path [default: .]

```

### Example

```
$ ./rcount -d ../../src
```

### Example to get total number of file with recursive

```
$ ./rcount -r -d ../../src
```

### Credits

- [StructOpt](https://github.com/TeXitoi/structopt)
- [WalkDir](https://github.com/BurntSushi/walkdir)
