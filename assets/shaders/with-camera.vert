#version 330

in vec3 position;
in vec3 color;

uniform mat4 camera;
uniform mat4 transform;

out vec3 vertexColor;

void main() {
    gl_Position = camera * transform * vec4(position, 1.0);
    vertexColor = color;
}
