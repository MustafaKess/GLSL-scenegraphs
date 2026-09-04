#version 430 core

out vec4 color;

uniform float time;

void main()
{
    float t = (sin(time * 0.5) + 1.0) / 2.0;

    vec3 colour = mix(
        vec3(0.0, 1.0, 0.0), // green
        vec3(1.0, 0.0, 0.0), // red
        t
    );

    color = vec4(colour, 1.0);
}