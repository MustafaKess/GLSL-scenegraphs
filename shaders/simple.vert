#version 430 core

in vec3 position;

void main()
{
    // Flip the scene horizontally and vertically
    gl_Position = vec4(-position.x, -position.y, position.z, 1.0f);
}