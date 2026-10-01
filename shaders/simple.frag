#version 430 core

in vec4 vertexColor;
in vec3 vertexNormal;

out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));

    float diffuse = max(
        0.0,
        dot(normalize(vertexNormal), -lightDirection)
    );

    color = vec4(
        vertexColor.rgb * diffuse,
        vertexColor.a
    );
}