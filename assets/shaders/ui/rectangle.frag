#version 330

out vec4 FragColor;

uniform vec4 color;

// x = top, y = right, z = bottom, w = left
uniform vec4 drawBounds;

bool is_within_draw_bounds(float top, float right, float bottom, float left) {
    return gl_FragCoord.y <= top &&
           gl_FragCoord.y >= bottom &&
           gl_FragCoord.x >= left &&
           gl_FragCoord.x <= right;
}

void main() {
    if (!is_within_draw_bounds(drawBounds.x, drawBounds.y, drawBounds.z, drawBounds.w)) {
        discard;
    }

    FragColor = vec4(color);
}
