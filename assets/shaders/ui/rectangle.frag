#version 330

out vec4 FragColor;

uniform vec4 color;
uniform vec4 drawBounds; // x = top, y = right, z = bottom, w = left

uniform vec4 borderColor;
uniform float borderSize;
uniform vec4 borderBounds; // x = top, y = right, z = bottom, w = left

bool is_within_bounds(float top, float right, float bottom, float left) {
    return gl_FragCoord.y <= top &&
           gl_FragCoord.y >= bottom &&
           gl_FragCoord.x >= left &&
           gl_FragCoord.x <= right;
}
bool is_within_bounds(vec4 bounds) {
    return gl_FragCoord.y <= bounds.x &&
           gl_FragCoord.y >= bounds.z &&
           gl_FragCoord.x >= bounds.w &&
           gl_FragCoord.x <= bounds.y;
}

void main() {
    if (!is_within_bounds(drawBounds)) {
        discard;
    }

    if (is_within_bounds(borderBounds)) {
        FragColor = vec4(color);
    } else {
        FragColor = vec4(borderColor);
    }
}
