#version 330

in vec2 position;

uniform float zIndex;
uniform vec2 view;
uniform vec2 worldPosition;
uniform vec2 scale;

void main() {    
    gl_Position = vec4(position * scale * view + worldPosition * view, zIndex, 1.0);
}
