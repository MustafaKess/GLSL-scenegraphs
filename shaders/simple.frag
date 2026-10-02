#version 430 core

in vec4 vertexColor;
in vec3 vertexNormal;
in vec3 vertexPosition;

out vec4 color;

void main()
{
    // Light direction
    vec3 lightDirection =
        normalize(vec3(0.8, -0.5, 0.6));

    // Surface normal
    vec3 normal =
        normalize(vertexNormal);

    // Direction from surface toward camera
    vec3 viewDirection =
        normalize(-vertexPosition);

    // Ambient
    float ambientStrength = 0.15;

    vec3 ambient =
        ambientStrength * vertexColor.rgb;

    // Diffuse
    float diffuse =
        max(
            0.0,
            dot(normal, -lightDirection)
        );

    vec3 diffuseColor =
        vertexColor.rgb * diffuse;

    // Specular
    vec3 reflectedLight =
        reflect(lightDirection, normal);

    float specular =
        pow(
            max(
                0.0,
                dot(viewDirection, reflectedLight)
            ),
            32.0
        );

    vec3 specularColor =
        vec3(1.0) * specular;


    // Final Phong color
    vec3 finalColor =
        ambient
        + diffuseColor
        + specularColor;

    color = vec4(
        finalColor,
        vertexColor.a
    );
}