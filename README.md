# TDT4195 – Computer Graphics Assignment 2

Implementation of **Assignment 2** for **TDT4195: Visual Computing Fundamentals** at NTNU.

This project builds on the previous Graphics Lab and introduces **per-vertex colors, alpha blending, affine transformation matrices, perspective projection, and a manually implemented camera system** using **Rust**, **OpenGL**, and **GLSL**. The implementation is based on the provided **Gloom-rs** starter project and the completed work from Assignment 1 / Graphics Lab 1.

Checkout [Gloom-rs](https://github.com/pbsds/gloom-rs) for the original starter project.

## Overview

The project covers the following concepts in real-time computer graphics:

- Per-vertex RGBA colors
- Vertex Buffer Objects (VBOs) for color attributes
- Passing vertex attributes between GLSL shaders
- Interpolation of vertex attributes across triangles
- Alpha blending
- Depth testing and the depth buffer
- The interaction between blending and draw/depth order
- 4×4 affine transformation matrices
- Identity, translation, scaling, and other affine transformations
- Passing transformation matrices from Rust to GLSL using uniforms
- Matrix multiplication and transformation order
- Perspective projection
- View frustums and clipping
- Building a camera transformation manually
- Camera translation along the X, Y, and Z axes
- Camera yaw and pitch rotation
- Frame-rate-independent camera movement using `delta_time`
- Combining multiple transformations into a single transformation matrix

The final implementation contains the completed state of the required tasks for Assignment 2.

## Technologies

- **Rust**
- **OpenGL 4.0 Core or higher**
- **GLSL**
- **Gloom-rs**
- **nalgebra-glm** (provided by Gloom-rs)

No additional external libraries are used beyond those provided with the Gloom-rs project.

## Assignment Tasks

### Task 1 – Per-Vertex Colors

The implementation extends the vertex data setup to support a separate color VBO containing an RGBA color for every vertex.

The vertex shader passes the color to the fragment shader, where OpenGL interpolates the colors across each triangle.

The rendered scene contains multiple triangles with different colors at their vertices.

### Task 2 – Alpha Blending and Depth

The implementation demonstrates how transparent triangles interact when they overlap at different depths.

The triangles:

- Have different colors
- Have transparent alpha values
- Occupy different Z depths
- Overlap in screen space
- Are rendered in back-to-front order

The implementation also demonstrates how changing triangle colors and depth positions affects the resulting blended color.

### Task 3 – Affine Transformation Matrix

A 4×4 transformation matrix is introduced into the vertex transformation pipeline.

The transformation matrix is used to demonstrate different affine transformations by modifying individual matrix elements.

The implementation also experiments with an animated value passed from Rust to GLSL through a uniform.

### Task 4 – Transformation Combinations and Camera

The transformation matrix is moved from the vertex shader into Rust.

The CPU constructs the complete transformation matrix each frame and sends the resulting matrix to the vertex shader as a uniform.

The implementation adds:

- Perspective projection
- Camera translation
- Camera yaw rotation
- Camera pitch rotation
- Combination of multiple transformation matrices
- Frame-rate-independent camera controls

The camera transformation is constructed manually rather than using `glm::look_at`.

The transformation order is handled explicitly because matrix multiplication is not commutative.

## Camera Controls

The camera follows the control convention specified by the assignment:

| Key | Movement |
|---|---|
| `W` | Forward |
| `S` | Backward |
| `A` | Left |
| `D` | Right |
| `Space` | Up |
| `Left Shift` | Down |
| `Left Arrow` | Yaw left |
| `Right Arrow` | Yaw right |
| `Up Arrow` | Pitch up |
| `Down Arrow` | Pitch down |

Camera movement is scaled using `delta_time` so that movement speed is independent of frame rate.

## Optional Bonus Tasks

The project may also contain implementations of the optional bonus challenges:

- Camera-relative movement
- Comparison of `smooth` and `noperspective` interpolation
- Camera-invariant billboards
- Analysis of the limitations of affine transformations

These tasks are optional and are not required for the normal assignment.

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