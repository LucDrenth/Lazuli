#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

void main() {
    // FragColor = texture(texture0, textureCoords);
    vec4 texColor = texture(texture0, textureCoords);
    FragColor = vec4(texColor.rgb * color, texColor.a);
}
