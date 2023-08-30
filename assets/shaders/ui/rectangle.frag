#version 330

out vec4 FragColor;

uniform vec4 color;
uniform vec4 drawBounds; // x = top, y = right, z = bottom, w = left

uniform vec4 borderColor;
uniform vec4 borderSize;
uniform vec4 borderBounds; // x = top, y = right, z = bottom, w = left

uniform vec4 elementBounds;
uniform vec4 borderRadius;

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

    // handle border radius
    // TODO anti aliasing
    vec2 borderRadiusCenterTopLeft = vec2(elementBounds.w + borderRadius.x, elementBounds.x - borderRadius.x);
    vec2 borderRadiusCenterTopRight = vec2(elementBounds.y - borderRadius.y, elementBounds.x - borderRadius.y);
    vec2 borderRadiusCenterBottomRight = vec2(elementBounds.y - borderRadius.z, elementBounds.z + borderRadius.z);
    vec2 borderRadiusCenterBottomLeft = vec2(elementBounds.w + borderRadius.w, elementBounds.z + borderRadius.w);

    if (gl_FragCoord.x < borderRadiusCenterTopLeft.x && gl_FragCoord.y > borderRadiusCenterTopLeft.y) {
        // top left corner
        float dist = distance(vec2(borderRadiusCenterTopLeft), gl_FragCoord.xy);
        if (dist > borderRadius.x) {
            discard;
        } else if (dist > borderRadius.x - min(borderSize.x, borderSize.w)) {
            FragColor = borderColor;
            return;
        }
    } else if (gl_FragCoord.x > borderRadiusCenterTopRight.x && gl_FragCoord.y > borderRadiusCenterTopRight.y) {
        // top right corner
        float dist = distance(vec2(borderRadiusCenterTopRight), gl_FragCoord.xy);
        if (dist > borderRadius.y) {
            discard;
        } else if (dist > borderRadius.y - min(borderSize.y, borderSize.x)) {
            FragColor = borderColor;
            return;
        }
    } else if (gl_FragCoord.x > borderRadiusCenterBottomRight.x && gl_FragCoord.y < borderRadiusCenterBottomRight.y) {
        // bottom right corner
        float dist = distance(vec2(borderRadiusCenterBottomRight), gl_FragCoord.xy);
        if (dist > borderRadius.z) {
            discard;
        } else if (dist > borderRadius.z - min(borderSize.y, borderSize.z)) {
            FragColor = borderColor;
            return;
        }
    } else if (gl_FragCoord.x < borderRadiusCenterBottomLeft.x && gl_FragCoord.y < borderRadiusCenterBottomLeft.y) {
        // bottom left corner
        float dist = distance(vec2(borderRadiusCenterBottomLeft), gl_FragCoord.xy);
        if (dist > borderRadius.w) {
            discard;
        } else if (dist > borderRadius.w - min(borderSize.z, borderSize.w)) {
            FragColor = borderColor;
            return;
        }
    }

    if (is_within_bounds(borderBounds)) {
        FragColor = color;
    } else {
        FragColor = borderColor;
    }
}
