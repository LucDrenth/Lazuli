#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;

void main() {
    FragColor = texture(texture0, textureCoords) * vec4(gl_FragCoord.x / 1500, gl_FragCoord.y / 1200, 0.85, 1.0);
}
