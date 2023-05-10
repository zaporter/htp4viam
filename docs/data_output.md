Data from a test is sent from the orchestrator into Elasticsearch.


This data has the following form:
Tests:
```json5
{
    id: "string",
    test_name: "string",
    creation_time: "timestamp",
    start_run_time: "timestamp",
    finished_time: "timestamp",
    test_config: // TODO
    dependencies: // TODO
}
```
Usage:
```json5
{
    id: "string",
    timestamp: "timestamp",
    cpu_load: float,
    ram_usage: float,
    // ... this should evolve from some 
    // generic sys-stats object
}
```

Logs:
```json5
{
    id: "string",
    timestamp: "timestamp",
    log_line: integer,
    data: "integer",
    is_stderr: bool
}
```


Custom: 
```json5
{
    id: "string",
    timestamp: "timestamp",
    data: jsonvalue
}
```


---
Dependency info
Dependency:
```json5
{
    id: "string",
    name: "string",
    built_for: "test_id string",
    build_start: "timestamp",
    build_end: "timestamp",
    save_location: "/path/to/file"
}
```
Maybe also collect util stats for building the dependency
