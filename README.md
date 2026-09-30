# shiny scenes (WIP)


Checkout [Gloom-rs](https://github.com/pbsds/gloom-rs) for the original starter project.



## Technologies

- **Rust**
- **OpenGL 4.0 Core or higher**
- **GLSL**
- **Gloom-rs**
- **nalgebra-glm** (provided by Gloom-rs)

No additional external libraries are used beyond those provided with the Gloom-rs project.


## Requirements

The project requires:

- Rust
- `cargo`
- `rustc`
- A GPU/driver supporting **OpenGL 4.0 Core or higher**

## Supported Operating Systems

The assignment is intended to be run on:

- Linux
- Windows

macOS is **not supported by the course**, due to unreliable OpenGL support.

## Project Structure

The project is based on the Gloom-rs structure. The main implementation is contained in the Rust source code and GLSL shader files, including:

- `main.rs`
- `shaders/simple.vert`
- `shaders/simple.frag`

The final source code represents the completed state of the assignment rather than separate implementations for each intermediate task.

## Notes

- OpenGL 4.0 Core or higher is required.
- OpenGL 4.3 or higher is recommended by the assignment.
- No additional external libraries should be added.
- Transformation matrices for Task 4 are created and combined on the CPU in Rust.
- The final combined transformation matrix is passed to the vertex shader through a uniform.
- The projection matrix must be the final transformation in the transformation chain.
- `glm::look_at` is not used; the camera transformation is constructed manually.