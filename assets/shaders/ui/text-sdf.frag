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
    
    float dist = texture(texture0, textureCoords).r;

    // TODO from uniform
    float fullColorThreshold = 0.5;
    float displayThreshold = 0.48;

    if (dist >= fullColorThreshold) {
        FragColor = vec4(color, 1.0);
    } 
    else if (dist >= displayThreshold) {
        float alpha = 1.0 - (fullColorThreshold - dist) / (fullColorThreshold - displayThreshold);
        FragColor = vec4(color, alpha);
    } 
    else {
        discard;
    }
}
