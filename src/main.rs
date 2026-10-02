// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/


//imports 
extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

//other modules
mod shader;
mod util;
mod mesh;
mod scene_graph;
mod toolbox;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


//VAO creation, vertices = actual vertex positions, 
// indices = the order in which the vertices are drawn
unsafe fn create_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
) -> u32 {
    let mut vao = 0;
    let mut vbo = 0;
    let mut cbo = 0;
    let mut ibo = 0;

    // VAO
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // =========================
    // Vertex positions
    // =========================

    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    // Attribute 0 = vertex position
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * size_of::<f32>(),
        ptr::null(),
    );

    gl::EnableVertexAttribArray(0);


    // =========================
    // Vertex normals
    // =========================

    let mut nbo = 0;

    gl::GenBuffers(1, &mut nbo);
    
    gl::BindBuffer(gl::ARRAY_BUFFER, nbo);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );

    // Attribute 2 = normal
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * size_of::<f32>(),
        ptr::null(),
    );

    gl::EnableVertexAttribArray(2);


    // Vertex colors

    gl::GenBuffers(1, &mut cbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, cbo);

    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(colors),
        pointer_to_array(colors),
        gl::STATIC_DRAW,
    );

    // Attribute 1 = color
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        4 * size_of::<f32>(),
        ptr::null(),
    );

    gl::EnableVertexAttribArray(1);


    // Indices
 
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    vao
}



unsafe fn draw_scene(
    node: &SceneNode,
    model_parent: &glm::Mat4,
    view_projection: &glm::Mat4,
    transform_location: i32,
    model_location: i32,
) {
    let translation =
        glm::translation(&node.position);

    let rotation_x =
        glm::rotation(
            node.rotation.x,
            &glm::vec3(1.0, 0.0, 0.0),
        );

    let rotation_y =
        glm::rotation(
            node.rotation.y,
            &glm::vec3(0.0, 1.0, 0.0),
        );

    let rotation_z =
        glm::rotation(
            node.rotation.z,
            &glm::vec3(0.0, 0.0, 1.0),
        );

    let reference_translation =
        glm::translation(&(-node.reference_point));

    let reference_translation_back =
        glm::translation(&node.reference_point);

    let node_transform =
        translation
        * reference_translation_back
        * rotation_z
        * rotation_y
        * rotation_x
        * reference_translation;

    // Accumulate only the Model transformation.
    let model =
        model_parent * node_transform;

    // View Projection × Model = MVP.
    let mvp =
        view_projection * model;

    if node.index_count >= 0 {
        gl::BindVertexArray(node.vao_id);

        // MVP: transforms vertex positions.
        gl::UniformMatrix4fv(
            transform_location,
            1,
            gl::FALSE,
            mvp.as_ptr(),
        );

        // Model: transforms vertex normals.
        gl::UniformMatrix4fv(
            model_location,
            1,
            gl::FALSE,
            model.as_ptr(),
        );

        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }

    for i in 0..node.n_children() {
        draw_scene(
            &node[i],
            &model,
            view_projection,
            transform_location,
            model_location,
        );
    }
}


fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));

    //pause functionality, for documentation purposes
    let arc_paused = Arc::new(Mutex::new(false));
    let paused = Arc::clone(&arc_paused);

    let arc_pause_cooldown = Arc::new(Mutex::new(std::time::Instant::now()));
    let pause_cooldown = Arc::clone(&arc_pause_cooldown);

    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }



        // == // Set up your VAO around here

let terrain = mesh::Terrain::load("./resources/lunarsurface.obj"); //task 1 wants us to load terrain using this file
let helicopter = mesh::Helicopter::load("./resources/helicopter.obj"); //task 2 for this one. 

let terrain_vao = unsafe {
    create_vao(
        &terrain.vertices,
        &terrain.indices,
        &terrain.colors,
        &terrain.normals,
    )
};

let helicopter_body_vao = unsafe {
    create_vao(
        &helicopter.body.vertices,
        &helicopter.body.indices,
        &helicopter.body.colors,
        &helicopter.body.normals,
    )
};

let helicopter_door_vao = unsafe {
    create_vao(
        &helicopter.door.vertices,
        &helicopter.door.indices,
        &helicopter.door.colors,
        &helicopter.door.normals,
    )
};

let helicopter_main_rotor_vao = unsafe {
    create_vao(
        &helicopter.main_rotor.vertices,
        &helicopter.main_rotor.indices,
        &helicopter.main_rotor.colors,
        &helicopter.main_rotor.normals,
    )
};

let helicopter_tail_rotor_vao = unsafe {
    create_vao(
        &helicopter.tail_rotor.vertices,
        &helicopter.tail_rotor.indices,
        &helicopter.tail_rotor.colors,
        &helicopter.tail_rotor.normals,
    )
};




let helicopter_body_index_count = helicopter.body.index_count;
let helicopter_door_index_count = helicopter.door.index_count;
let helicopter_main_rotor_index_count = helicopter.main_rotor.index_count;
let helicopter_tail_rotor_index_count = helicopter.tail_rotor.index_count;

let index_count = terrain.index_count;



let mut scene_root = SceneNode::new();

let mut terrain_node =
    SceneNode::from_vao(terrain_vao, index_count);

// Store the five helicopter root nodes.
// All helicopters share the same VAOs.
let mut helicopter_roots: Vec<scene_graph::Node> =
    Vec::new();

for _ in 0..5 {
    let mut helicopter_root =
        SceneNode::new();

    let mut helicopter_body_node =
        SceneNode::from_vao(
            helicopter_body_vao,
            helicopter_body_index_count,
        );

    let mut helicopter_door_node =
        SceneNode::from_vao(
            helicopter_door_vao,
            helicopter_door_index_count,
        );

    let mut helicopter_main_rotor_node =
        SceneNode::from_vao(
            helicopter_main_rotor_vao,
            helicopter_main_rotor_index_count,
        );

    let mut helicopter_tail_rotor_node =
        SceneNode::from_vao(
            helicopter_tail_rotor_vao,
            helicopter_tail_rotor_index_count,
        );

    // Reference points
    helicopter_root.reference_point =
        glm::vec3(0.0, 0.0, 0.0);

    helicopter_body_node.reference_point =
        glm::vec3(0.0, 0.0, 0.0);

    helicopter_door_node.reference_point =
        glm::vec3(0.0, 0.0, 0.0);

    helicopter_main_rotor_node.reference_point =
        glm::vec3(0.0, 0.0, 0.0);

    helicopter_tail_rotor_node.reference_point =
        glm::vec3(0.35, 2.3, 10.4);

    // Helicopter hierarchy
    helicopter_root.add_child(
        &helicopter_body_node
    );

    helicopter_root.add_child(
        &helicopter_door_node
    );

    helicopter_root.add_child(
        &helicopter_main_rotor_node
    );

    helicopter_root.add_child(
        &helicopter_tail_rotor_node
    );

    // Store this helicopter root.
    helicopter_roots.push(helicopter_root);
}

// Scene root and terrain reference points
scene_root.reference_point =
    glm::vec3(0.0, 0.0, 0.0);

terrain_node.reference_point =
    glm::vec3(0.0, 0.0, 0.0);

// Add all five helicopters to the terrain
for helicopter_root in &helicopter_roots {
    terrain_node.add_child(&*helicopter_root);
}

// Terrain is the child of the scene root
scene_root.add_child(&terrain_node);


//implementation of the shader builder is in src/shader.rs
let simple_shader = unsafe { 
    shader::ShaderBuilder::new()
        .attach_file("./shaders/simple.vert")
        .attach_file("./shaders/simple.frag")
        .link()
};



/*
// allowing transform value (task 3, assignment 2)
let transform_value_location = unsafe {
    simple_shader.get_uniform_location("transformValue")
};
*/

let transform_location = unsafe {
    simple_shader.get_uniform_location("transform")
};

let model_location = unsafe {
    simple_shader.get_uniform_location("model")
};


/*
let time_location = unsafe { //needed for extra challange d (assignment 1), for color changing

    simple_shader.get_uniform_location("time")
};
*/


        // Used to demonstrate keyboard handling for exercise 2.
        //let mut _arbitrary_number = 0.0; // feel free to remove


        //Task 4, assignment 2
        // Camera position
        let mut camera_x: f32 = 0.0;
        let mut camera_y: f32 = 0.0;
        let mut camera_z: f32 = 0.0;

        // Camera rotation
        let mut camera_yaw: f32 = 0.0;
        let mut camera_pitch: f32 = 0.0;

        // Camera movement speed.
        // 50 = normal fast movement, 500 = running speed.
        let mut movement_speed: f32 = 50.0;

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }



            // Changes done after bonus task
            //let mut movement = glm::vec3(0.0, 0.0, 0.0);

            // Changes done after bonus task
            let mut movement = glm::vec3(0.0, 0.0, 0.0);

            if let Ok(keys) = pressed_keys.lock() {

                // Hold R for running speed.
                // Normal speed = 50
                // Running speed = 500
                let movement_speed =
                    if keys.contains(&VirtualKeyCode::R) {
                        300.0
                    } else {
                        50.0
                    };

                for key in keys.iter() {
                    match key {

                        VirtualKeyCode::A => {
                            movement.x -= movement_speed * delta_time;
                        }

                        VirtualKeyCode::D => {
                            movement.x += movement_speed * delta_time;
                        }

                        VirtualKeyCode::W => {
                            movement.z -= movement_speed * delta_time;
                        }

                        VirtualKeyCode::S => {
                            movement.z += movement_speed * delta_time;
                        }

                        VirtualKeyCode::Space => {
                            movement.y += movement_speed * delta_time;
                        }

                        VirtualKeyCode::LShift => {
                            movement.y -= movement_speed * delta_time;
                        }

                        VirtualKeyCode::Left => {
                            camera_yaw -= delta_time;
                        }

                        VirtualKeyCode::Right => {
                            camera_yaw += delta_time;
                        }

                        VirtualKeyCode::Up => {
                            camera_pitch += delta_time;
                        }

                        VirtualKeyCode::Down => {
                            camera_pitch -= delta_time;
                        }

                        _ => {}
                    }
                }
            }
            

            // Limit vertical camera rotation
            let max_pitch = glm::radians(&glm::vec1(89.0)).x;

            if camera_pitch > max_pitch {
                camera_pitch = max_pitch;
            }

            if camera_pitch < -max_pitch {
                camera_pitch = -max_pitch;
            }
            
            let movement_rotation =
                glm::rotation(
                    camera_yaw,
                    &glm::vec3(0.0, 1.0, 0.0),
                ) *
                glm::rotation(
                    camera_pitch,
                    &glm::vec3(1.0, 0.0, 0.0),
                );

            let world_movement =
                movement_rotation *
                glm::vec4(
                    movement.x,
                    movement.y,
                    movement.z,
                    0.0,
                );

            camera_x += world_movement.x;
            camera_y += world_movement.y;
            camera_z += world_movement.z;
                        




            /*
            // Keybinds BEFORE CHANGES DUE TO BONUS TASK 5A
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {

                        // Camera movement along X axis
                        VirtualKeyCode::A => {
                            camera_x -= 2.0 * delta_time;
                        }
                        VirtualKeyCode::D => {
                            camera_x += 2.0 * delta_time;
                        }

                        // Camera movement along Z axis
                        VirtualKeyCode::W => {
                            camera_z -= 2.0 * delta_time;
                        }
                        VirtualKeyCode::S => {
                            camera_z += 2.0 * delta_time;
                        }

                        // Camera movement along Y axis
                        VirtualKeyCode::Space => {
                            camera_y += 2.0 * delta_time;
                        }
                        VirtualKeyCode::LShift => {
                            camera_y -= 2.0 * delta_time;
                        }

                        // Camera horizontal rotation
                        VirtualKeyCode::Left => {
                            camera_yaw -= delta_time;
                        }
                        VirtualKeyCode::Right => {
                            camera_yaw += delta_time;
                        }

                        // Camera vertical rotation
                        VirtualKeyCode::Up => {
                            camera_pitch += delta_time;
                        }
                        VirtualKeyCode::Down => {
                            camera_pitch -= delta_time;
                        }

                        _ => { }
                    }
                }
            }
            */


            // Task 4a + 4b + Task 6a:
            // Animate all helicopters unless paused.
            let is_paused = {
                if let Ok(paused_state) = paused.lock() {
                    *paused_state
                } else {
                    false
                }
            };

            if !is_paused {
                for (i, helicopter_root)
                    in helicopter_roots.iter_mut().enumerate()
                {
                    // Each helicopter gets a different point along
                    // the same animation path.
                    let offset = i as f32 * 1.5;

                    let helicopter_time =
                        elapsed + offset;

                    let heading =
                        toolbox::simple_heading_animation(
                            helicopter_time
                        );

                    // Follow the same path with a different offset.
                    helicopter_root.position =
                        glm::vec3(
                            heading.x,
                            0.0,
                            heading.z,
                        );

                    // Follow the path orientation.
                    helicopter_root.rotation.x =
                        heading.pitch;

                    helicopter_root.rotation.y =
                        heading.yaw;

                    helicopter_root.rotation.z =
                        heading.roll;

                    // Main rotor
                    helicopter_root[2].rotation.y =
                        helicopter_time * 5.0;

                    // Tail rotor
                    helicopter_root[3].rotation.x =
                        helicopter_time * 5.0;
                }
            }



            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)

            // Create the camera transformation from scratch every frame.
            let mut camera_transform: glm::Mat4 = glm::identity();

            // Move the world in the opposite direction of the camera.

            let scene_translation =
                glm::translation(&glm::vec3(0.0, 0.0, -2.0)); // Keeps triangles infront of camera

            let camera_translation =
                glm::translation(&glm::vec3(
                    -camera_x,
                    -camera_y,
                    -camera_z,
                ));

            // Rotate the world in the opposite direction of the camera.
            let yaw_rotation =
                glm::rotation(
                    -camera_yaw,
                    &glm::vec3(0.0, 1.0, 0.0),
                );

            let pitch_rotation =
                glm::rotation(
                    -camera_pitch,
                    &glm::vec3(1.0, 0.0, 0.0),
                );

            // Combine the camera transformations.
                camera_transform =
                    pitch_rotation *
                    yaw_rotation *
                    camera_translation *
                    scene_translation;

            let projection: glm::Mat4 =
                glm::perspective(
                    window_aspect_ratio,
                    glm::radians(&glm::vec1(45.0)).x,
                    1.0,
                    2000.0, //assingment 3 change
                    
            );

                // Projection must be the final transformation.
            let transform: glm::Mat4 = projection * camera_transform;


            unsafe {
                simple_shader.activate();

                gl::ClearColor(0.035, 0.046, 0.078, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                draw_scene(
                    &scene_root,
                    &glm::identity(),
                    &transform,
                    transform_location,
                    model_location,
                );
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => { //changed to allow pausing (and buffer for the pause/unpause so it doesnt swap too fast)
                            if !keys.contains(&keycode) {
                                keys.push(keycode);

                                if keycode == P {
                                    if let Ok(mut last_toggle) = pause_cooldown.lock() {
                                        let now = std::time::Instant::now();

                                        // 500 ms cooldown between pause/unpause
                                        if now.duration_since(*last_toggle).as_millis() >= 300 {
                                            *last_toggle = now;

                                            if let Ok(mut paused_state) = arc_paused.lock() {
                                                *paused_state = !*paused_state;
                                                println!("Helicopter paused: {}", *paused_state);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }

                    P => {
                        if let Ok(mut paused_state) = arc_paused.lock() {
                            *paused_state = !*paused_state;
                            println!("Helicopter paused: {}", *paused_state);
                        }
                    }
                    

                    _ => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
