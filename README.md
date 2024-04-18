# About
Lazuli is an engine for 3d graphics.

# How-to
## Create a new project
To create a new project using this engine, run [run script](./scripts/create-project.sh) with the project name as the second argument.
For example,
```bash
./scripts/create-project.sh My-awesome-project
```

The project will be created in the same parent directory that this repository resides in.
For example:

    home
    ├── Lazuli
    └── My-awesome-project

# Contributing
## Running a test scene
To start the engine locally, run [run.sh](/scripts/run.sh). The scene that will be used is defined in [the entrypoint file](./src/bin.rs).

## Coding standards
### Shaders
Local variables names use the snake_case convention. Variable qualifiers ('in', 'out', 'uniform' etc.) use the camelCase convention.

### Scripts
Variables should be screaming snake_case, like 
```bash
MY_VARIABLE_NAME="awesome"
```
