#version 330

in vec3 position;
in vec2 vertexTextureCoordinates;

out vec2 textureCoords;

uniform vec2 view;
uniform vec2 worldPosition;

void main() {    
    gl_Position = vec4(position.xy * view + worldPosition * view, position.z, 1.0);
    textureCoords = vertexTextureCoordinates;
}
