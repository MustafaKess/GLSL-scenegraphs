#version 430 core

in vec4 vertexColor;
in vec3 vertexNormal;

out vec4 color;

void main()
{
    color = vec4(vertexNormal, 1.0);
}