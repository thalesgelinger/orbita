# Orbita
<img src="https://github.com/user-attachments/assets/c200ced3-6884-4b3f-b209-f72dc5119354" alt="Orbite Logo" width="200"/>

Orbite is a simple and beautiful package manager for Lua. It helps you manage your Lua projects and their dependencies directly from GitHub with minimal configuration. It is designed to make Lua project setup and execution easier, especially when dealing with dependency management.

## Features

- Start a new project: Easily initialize a new Lua project with the orbita start command.
- Dependency management: Resolve and install dependencies directly from GitHub using the orbita resolve and orbita install commands.
- Run projects with resolved dependencies: No need to worry about modifying package.path—just run your project with orbita run.
- Customizable configuration: Define your project's dependencies in a Lua configuration file (Orbite file).

## Installation

To install and build Orbite, follow these steps:

1. Clone the repository:
```bash
git clone https://github.com/your-username/orbita.git
cd orbita
```

2. Build the project:

If you have Makefile set up, you can run the build command:
```bash
make
```

This will:

- Bundle Lua code using lua bundle.lua
- Compile the Lua code into a binary with luajit
- Create a header file (main_lua.h) from the binary
- Compile the final program with GCC

Clean the build (optional):
```bash
make clean
```

## Usage

Once Orbite is installed, you can use the following commands:
```bash
orbita start
```

Starts a new empty Orbite project. This will create a basic Orbite configuration file (Orbite.lua).
```bash
orbita resolve
```
Resolves and installs your project's dependencies. Dependencies are defined in the Orbite.lua file.
```bash
orbita run <file>
```
Runs the project with the resolved dependencies. No need to modify package.path—Orbite will handle it for you.
```bash 
orbita install <repo> <version>
```

Installs a specific package from GitHub.
Example Orbite configuration

Here is an example configuration file for a Lua project managed by Orbite:
```lua
return {
    name = "my-awesome-project",
    version = "1.0.0",
    description = "my awesome project description",
    dependencies = {
        { "github.com/lunarmodules/luasocket" }
    },
    author = "Thales Gelinger",
    license = "MIT"
}
```

In this example, the project "farol" depends on the luasocket module, which is defined with a custom path.
License

This project is licensed under the MIT License. See the LICENSE file for more details.
