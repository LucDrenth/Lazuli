#version 330

in vec2 position;
in vec2 textureCoordinates;

out vec2 textureCoords;

uniform float zIndex;
uniform vec2 view;
uniform vec2 worldPosition;
uniform vec2 scale;

void main() {    
    gl_Position = vec4(position * scale * view + worldPosition * view, zIndex, 1.0);
    textureCoords = textureCoordinates;
}
