#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;

out vec4 vertexColor;

uniform float transformValue;

float a = 1.0;
float b = 0.0;
float c = 0.0;

float d = 0.0;
float e = 1.0;
float f = 0.0;

void main()
{
    mat4 transform;

    // Transformation matrix
    transform[0] = vec4(a, d, 0.0, 0.0);
    transform[1] = vec4(b, e, 0.0, 0.0);
    transform[2] = vec4(0.0, 0.0, 1.0, 0.0);
    transform[3] = vec4(c, f, 0.0, 1.0);

    gl_Position = transform * vec4(position, 1.0);

    vertexColor = color;
}