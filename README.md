# JsonlTools
Tools for filtering and manipulating JSONL

# Tools
## jsonl_tool
This tool uses JMESPath queries to filter or transform jsonl input.

```
jsonl_tool 0.1.0
Matthew Seyer <https://github.com/forensicmatt/JsonlTools>
Tool to filter and format JSONL with JMESPath queries.

USAGE:
    jsonl_tool.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --debug <DEBUG>              Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
    -d, --delimiter <DELIMITER>      The delimiter to use if result is an array.
    -f, --filter <JMES_FILTER>...    The JMESPath filter to use.
    -s, --source <PATH>              The source of the jsonl file.
    -t, --text <TEXT_PATTERN>        The text pattern to use. (Must be a JMESPath query that results in an array)
```
