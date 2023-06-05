#version 330

in vec3 position;
in vec2 vertexTextureCoordinates;

uniform mat4 transform;

out vec2 textureCoords;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    textureCoords = vertexTextureCoordinates;
}
