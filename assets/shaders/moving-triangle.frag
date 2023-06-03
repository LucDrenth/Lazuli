#version 330

in vec3 vertexColor;

out vec4 fragColor;

void main() {
    fragColor = vec4(gl_FragCoord.x / 1500, gl_FragCoord.y / 1000, vertexColor.z, 1.0);
}
