# Platform Orchestrator

This is the brains of the entire platform. 

## Core responsibilities:
- Run tests
  - Read test definitions
  - Update dependencies and re-run tests when dependencies update
  - Maintain a test queue that specifies the dependencies for each test
- Seperate automated runs from manual runs 
- Never fail or enter an irrecoverable/non-responsive state
- Export data in a way that is useful for the visualizing software
- Manage testing hardware
  - Save/serve snapshots
  - Connect hardware with apparatuses
- Interface with a user via gRPC calls
  

## Core concepts:
**DependencyManager**: Responsible for creating the external dependency graph

**SnapshotManager**: Manages snapshots

**TestExecutor**: Manages a single test

**TestPlanner**: Plans how to run tests in a way that minimizes time
