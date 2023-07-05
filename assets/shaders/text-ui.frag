#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

void main() {
    float distance = texture(texture0, textureCoords).r;

    // TODO from uniform
    float threshold = 0.5;

    float alpha = smoothstep(threshold - 0.1, threshold + 0.1, distance);
    FragColor = vec4(color, alpha);
}
