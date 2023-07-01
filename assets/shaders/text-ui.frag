#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

void main() {
    vec4 textureColor = texture(texture0, textureCoords);
    FragColor = vec4(textureColor.rgb * color, textureColor.a);
}
