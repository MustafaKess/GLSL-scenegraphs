#version 430 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec3 normal;

out vec4 vertexColor;
out vec3 vertexNormal;

uniform mat4 transform;

void main()
{
    gl_Position = transform * vec4(position, 1.0);

    vertexColor = color;
    vertexNormal = normal;
}