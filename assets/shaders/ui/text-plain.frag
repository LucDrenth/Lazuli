#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

void main() {
    float alpha = texture(texture0, textureCoords).r;
    FragColor = vec4(color, alpha);
}
