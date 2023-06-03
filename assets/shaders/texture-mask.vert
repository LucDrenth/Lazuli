#version 330

in vec3 position;
in vec2 vertexTextureCoordinates;

out vec2 textureCoords;

void main() {
    gl_Position = vec4(position, 1.0);
    textureCoords = vertexTextureCoordinates;
}
