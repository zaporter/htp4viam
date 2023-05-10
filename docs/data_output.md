Each test is given an owned stream into Elasticsearch when it is created. 

## Test Data:
### `tests` Index
```json5
{
    t_id: "my-test-7d8ds0csfmsdkkf3e (UUID)", 
    test_name: "my-test",
    creation_time: "timestamp",
    execution_start_time: "timestamp",
    termination_time: "timestamp",
    // True if the test's script has a return code of 0. False otherwise
    passed: bool,
    stage_success: {
        validation: bool,
        dependency_building: bool,
        resource_aquision: bool,
        // This does not indicate if the test succeeded or failed, 
        // but rather if it was executed without errors
        execution: bool,
    },
    // The config of the executed test
    test_config: jsonvalue,
    // Dependencies that were used (or built) for this test
    dependencies: ["dependency uuid", ...],
}
```
### `utilization` Index
Resource utilization of the underlying hardware. This may not be reliable if the test is run in a docker container as other processes can interfere with resource utilization.
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    cpu_load: float,
    ram_usage: float,
    // TODO ... this should be created from some 
    // generic sys-stats object
}
```

### `logs` Index
stdout + stderr of the test during execution
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    log_line: postitive incrementing integer,
    data: "Log message string",
    is_stderr: bool,
}
```

### `stats` Index
This stores custom test-specific statistics that might be useful for different tests

Ex: a camera test may wish to store and graph frame-latency or jitter
```json5
{
    t_id: "test uuid",
    timestamp: "timestamp",
    data: jsonvalue,
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
    fs_root: "/path/to/dependency",
}
```

## Meta htp related info
Todo. Most data will go through loki via promtail.
