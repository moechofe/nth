A command-line tool that returns extracted columns from stdin.
Aims to replace `awk {print $1}` with something easier to use.

### Usage

    cat file | nth 0

This will extract the first column of file, equivalent to:

    cat file | awk '{print $1}'

When a number is found in the command arguments, it will fetch the corresponding entry from STDIN. If any other string is found, it will echo it back.

    cat file | nth "second" 1 "forth" 3

### Build

    cargo build -r
