#version 330

in vec2 position;

uniform float zIndex;
uniform vec2 view;
uniform vec2 worldPosition;

void main() {    
    gl_Position = vec4(position.xy * view + worldPosition * view, zIndex, 1.0);
}
