#version 330

in vec2 position;
in vec2 textureCoordinates;

out vec2 textureCoords;

uniform float zIndex;
uniform vec2 view;
uniform vec2 worldPosition;
uniform vec2 scale;

uniform float rotation;

void main() {
    mat2 rotationMatrix = mat2(cos(rotation), -sin(rotation), sin(rotation), cos(rotation));
    vec2 rotatedPosition = rotationMatrix * (position * scale);
    vec2 finalPosition = (rotatedPosition + worldPosition) * view;

    gl_Position = vec4(finalPosition, zIndex, 1.0);
    textureCoords = textureCoordinates;
}
