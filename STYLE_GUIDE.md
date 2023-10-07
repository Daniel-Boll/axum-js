# `@lambda-group/axum-js` Style Guide

This document outlines the coding conventions of the `@lambda-group/axum-js` project. Adhering to these guidelines helps maintain a consistent, readable codebase.

## Rust

Follow the [official Rust style guide](https://doc.rust-lang.org/1.0.0/style/README.html) and the project's `rustfmt.toml` configuration file.

### Directory Structure

- `src/`
   - `lib.rs`: This file should only be used to include sub-modules.
   - `<module_path>/`
     - `mod.rs`: Define structures and add the dependencies files of this module here.
     - `module_file_1.rs`: Implement module-specific logic here.
     - `module_file_2.rs`: Implement module-specific logic here.
     - ...

### Naming Conventions

- Use snake_case for file names, e.g., `my_module.rs`.
- Use CamelCase for type names, e.g., `MyStruct`.

### Error Handling

- Prefer using `Result<T, E>` for functions that can fail.
- Use `expect` only when it's logically impossible for the operation to fail.

## JavaScript (Testing)

Follow the existing JavaScript conventions and utilize the `deno fmt` for formatting your JavaScript test files.

### Naming Conventions

- Use camelCase for variables and function names, e.g., `myFunction`.
- Use PascalCase for classes, e.g., `MyClass`.

### Error Handling

- Prefer using try-catch blocks for handling exceptions.
- Always handle promise rejections by using `.catch()` method or `try-catch` block with `await`.

### Testing

- Prefer using descriptive test case names.
- Organize tests logically, and keep related tests grouped together.

Thank you for adhering to `@lambda-group/axum-js`'s style guidelines, ensuring a cohesive and well-structured codebase!
