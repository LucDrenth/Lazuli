#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform sampler2D texture1;

void main() {
    vec4 color0 = texture(texture0, textureCoords);
    vec4 color1 = texture(texture1, textureCoords);

    FragColor = mix(color0, color1, 0.6) * color0.a;
}
