#version 330

in vec3 position;
in vec3 color;

uniform float xPos;

out vec3 vertexColor;

void main() {
    gl_Position = vec4(position.x + xPos, position.yz , 1.0);
    vertexColor = color;
}
