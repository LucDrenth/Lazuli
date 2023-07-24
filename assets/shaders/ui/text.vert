#version 330

in vec2 position;
in vec2 vertexTextureCoordinates;

out vec2 textureCoords;

uniform float zIndex;
uniform vec2 view;
uniform vec2 worldPosition;

void main() {    
    gl_Position = vec4(position * view + worldPosition * view, zIndex, 1.0);
    textureCoords = vertexTextureCoordinates;
}
