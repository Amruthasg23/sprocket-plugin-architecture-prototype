
# Sprocket Plugin Architecture Prototype

This repository contains a prototype demonstrating a **plugin-based execution backend architecture** written in Rust.

The goal of this prototype is to explore how execution backends can be designed as **pluggable components**, allowing different execution strategies to be integrated without modifying the core system.

## Motivation

Modern build and execution systems often need to support multiple execution environments such as:

- Local execution
- Containerized execution
- Remote server execution

A plugin-based architecture allows new execution backends to be added easily while maintaining a clean separation between the core system and backend implementations.

## Implemented Backends

This prototype currently includes three backend implementations:

- **LocalBackend** – executes jobs on the local system
- **DockerBackend** – represents execution inside a container environment
- **RemoteBackend** – represents execution on a remote machine

Each backend implements a common interface defined by the `ExecutionBackend` trait.

## Project Structure
src/ 
├── main.rs 
├── plugin_interface.rs 
├── sample_backend.rs 
├── docker_backend.rs 
└── remote_backend.rs


## Core Idea

The architecture is based on a trait:ExecutionBackend ,which defines the interface that all execution backends must implement.

This allows the system to treat all backends uniformly while enabling different execution strategies.

## Example Output
Run the project using:

cargo run

Example Output:
Loaded backend: LocalBackend Executing job 'example_job' on LocalBackend
Loaded backend: DockerBackend Executing job 'example_job' inside Docker container
Loaded backend: RemoteBackend Executing job 'example_job' on Remote Server


## Purpose of This Prototype

This project explores how pluggable execution backends could be implemented in Rust for systems that require flexible execution strategies.

It serves as an experimental foundation for designing extensible execution architectures.
