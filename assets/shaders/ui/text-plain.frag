#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

// x = top, y = right, z = bottom, w = left
uniform vec4 drawBounds;

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
    
    float alpha = texture(texture0, textureCoords).r;
    FragColor = vec4(color, alpha);
}
