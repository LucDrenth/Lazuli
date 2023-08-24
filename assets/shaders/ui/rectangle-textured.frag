#version 330

in vec2 textureCoords;

out vec4 FragColor;

uniform vec4 color;
uniform sampler2D texture0;
uniform vec4 drawBounds; // x = top, y = right, z = bottom, w = left

void main() {
    float bound_top = drawBounds.x;
    float bound_right = drawBounds.y;
    float boud_bottom = drawBounds.z;
    float bound_left = drawBounds.w;
    
    if (gl_FragCoord.y > bound_top || 
        gl_FragCoord.y < boud_bottom || 
        gl_FragCoord.x < bound_left || 
        gl_FragCoord.x > bound_right
    ) {
        discard;
    }

    vec4 texture_color = texture(texture0, textureCoords);

    if (texture_color.w < 1.0) {
        // This does not give the desired result, but it should be something like this, but weighted
        // FragColor = (texture_color + color) / 2;

        // TODO  mix with texture_color
        FragColor = color;
    } else {
        FragColor = texture_color;
    }
}
