# JsonlTools
Tools for filtering and manipulating JSONL

# Tools
## filter_jsonl
This tool uses JMESPath queries to filter or transform jsonl input.

```
filter_jsonl 0.1.0
Matthew Seyer <https://github.com/forensicmatt/JsonlTools>
Tool to filter JSONL with JMESPath queries.

USAGE:
    filter_jsonl.exe [FLAGS] [OPTIONS]

FLAGS:
    -b, --bool_expr    JMES Query as bool only. (Prints whole record if true.)
    -h, --help         Prints help information
    -p, --pipe         Read from STDIN pipe.
    -V, --version      Prints version information

OPTIONS:
    -d, --debug <DEBUG>           Debug level to use. [possible values: Off, Error, Warn, Info, Debug, Trace]
    -f, --filter <JMES_FILTER>    The JMESPath filter to use.
    -s, --source <PATH>           The source of the jsonl file.
```

### Example
#### Filter
Run a tool that produces jsonl output (like rusty_usn), then, pipe output to filter_jsonl.
```
D:\Testing>rusty_usn.exe -s G:\Images\CTF_Defcon2018\03_Desktop\KAPE\K\$Extend\$J | filter_jsonl.exe -p -b -f "ends_with(file_name, '.pf')"
```

Output is records where the file_name attribute ends with '.pf':
```
{"_source":"G:\\Images\\CTF_Defcon2018\\03_Desktop\\KAPE\\K\\$Extend\\$J","_offset":9392,"record_length":120,"major_version":2,"minor_version":0,"file_reference":{"entry":255696,"sequence":5},"parent_reference":{"entry":169895,"sequence":7},"usn":1231037616,"timestamp":"2018-07-30T20:01:36.975233Z","reason":"USN_REASON_DATA_TRUNCATION","source_info":"(empty)","security_id":0,"file_attributes":8224,"file_name_length":58,"file_name_offset":60,"file_name":"RUNTIMEBROKER.EXE-5C74CC5C.pf"}
{"_source":"G:\\Images\\CTF_Defcon2018\\03_Desktop\\KAPE\\K\\$Extend\\$J","_offset":9512,"record_length":120,"major_version":2,"minor_version":0,"file_reference":{"entry":255696,"sequence":5},"parent_reference":{"entry":169895,"sequence":7},"usn":1231037736,"timestamp":"2018-07-30T20:01:36.975233Z","reason":"USN_REASON_DATA_EXTEND | USN_REASON_DATA_TRUNCATION","source_info":"(empty)","security_id":0,"file_attributes":8224,"file_name_length":58,"file_name_offset":60,"file_name":"RUNTIMEBROKER.EXE-5C74CC5C.pf"}
{"_source":"G:\\Images\\CTF_Defcon2018\\03_Desktop\\KAPE\\K\\$Extend\\$J","_offset":9632,"record_length":120,"major_version":2,"minor_version":0,"file_reference":{"entry":255696,"sequence":5},"parent_reference":{"entry":169895,"sequence":7},"usn":1231037856,"timestamp":"2018-07-30T20:01:36.975233Z","reason":"USN_REASON_CLOSE | USN_REASON_DATA_EXTEND | USN_REASON_DATA_TRUNCATION","source_info":"(empty)","security_id":0,"file_attributes":8224,"file_name_length":58,"file_name_offset":60,"file_name":"RUNTIMEBROKER.EXE-5C74CC5C.pf"}
{"_source":"G:\\Images\\CTF_Defcon2018\\03_Desktop\\KAPE\\K\\$Extend\\$J","_offset":15520,"record_length":128,"major_version":2,"minor_version":0,"file_reference":{"entry":255541,"sequence":5},"parent_reference":{"entry":169895,"sequence":7},"usn":1231043744,"timestamp":"2018-07-30T20:07:19.256484Z","reason":"USN_REASON_DATA_TRUNCATION","source_info":"(empty)","security_id":0,"file_attributes":8224,"file_name_length":68,"file_name_offset":60,"file_name":"SEARCHPROTOCOLHOST.EXE-AFAD3EF9.pf"}
```

#### Transform
Output array of timestamp and file_name attributes.
```
D:\Testing>rusty_usn.exe -s G:\Images\CTF_Defcon2018\03_Desktop\KAPE\K\$Extend\$J | filter_jsonl.exe -p -f "[timestamp, file_name]"
["2018-07-30T19:49:10.115811Z","1532980150.~tmp"]
["2018-07-30T19:49:10.147082Z","1532980150.~tmp"]
["2018-07-30T19:49:10.147082Z","1532980150.~tmp"]
["2018-07-30T19:49:10.147082Z","1532980150.~tmp"]
["2018-07-30T19:49:10.147082Z","1532980150~RF60cce368.TMP"]
["2018-07-30T19:49:10.147082Z","1532980150~RF60cce368.TMP"]
["2018-07-30T19:49:10.147082Z","1532980150~RF60cce368.TMP"]
["2018-07-30T19:49:10.147082Z","1532980150"]
["2018-07-30T19:49:10.147082Z","1532980150~RF60cce368.TMP"]
["2018-07-30T19:49:10.147082Z","1532980150.~tmp"]
["2018-07-30T19:49:10.147082Z","1532980150"]
["2018-07-30T19:49:10.147082Z","1532980150~RF60cce368.TMP"]
["2018-07-30T19:49:10.147082Z","1532980150"]
...
```
