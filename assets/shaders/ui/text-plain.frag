#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

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
    
    float alpha = texture(texture0, textureCoords).r;
    FragColor = vec4(color, alpha);
}
