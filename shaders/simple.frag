#version 430 core

in vec3 vertexNormal;

out vec4 color;

void main()
{
    color = vec4(normalize(vertexNormal) * 0.5 + 0.5, 1.0);
}