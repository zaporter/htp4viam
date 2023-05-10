Each test is given an owned stream into Elasticsearch when it is created. 

## Test Data:
### `tests` Index
```json5
{
    t_id: "my-test-7d8ds0csfmsdkkf3e (UUID)", 
    test_name: "my-test",
    creation_time: "timestamp",
    start_run_time: "timestamp",
    finished_time: "timestamp",
    test_config: jsonvalue
    dependencies: ["dependency uuid", ...]
}
```
### `utilization` Index
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    cpu_load: float,
    ram_usage: float,
    // ... this should evolve from some 
    // generic sys-stats object
}
```

### `logs` Index
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    log_line: postitive incrementing integer,
    data: "Log message string",
    is_stderr: bool
}
```

### `stats` Index
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    data: jsonvalue
}
```


## Dependencies
### `dependencies` Index
```json5
{
    d_id: "dependency uuid",
    name: "string",
    built_for: "test uuid",
    build_start: "timestamp",
    build_end: "timestamp",
    fs_root: "/path/to/file",
}
```
Maybe also collect util stats for building the dependency
