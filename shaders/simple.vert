#version 430 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;

out vec4 vertexColor;
out vec3 vertexNormal;
out vec3 vertexPosition;

uniform mat4 transform;
uniform mat4 model;

void main()
{
    gl_Position = transform * vec4(position, 1.0);

    vertexColor = color;
    vertexNormal = normalize(mat3(model) * normal);
    vertexPosition = vec3(model * vec4(position, 1.0));
    }