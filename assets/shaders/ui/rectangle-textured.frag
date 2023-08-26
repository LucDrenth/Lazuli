#version 330

in vec2 textureCoords;

out vec4 FragColor;

uniform vec4 color;
uniform sampler2D texture0;
uniform vec4 drawBounds; // x = top, y = right, z = bottom, w = left

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

    vec4 texture_color = texture(texture0, textureCoords);
    float bg_color_alpha_contribution = (1.0 - texture_color.w) * color.w;

    vec3 frag_from_texture = texture_color.xyz * texture_color.w;
    vec3 frag_from_color = color.xyz * bg_color_alpha_contribution;
    float frag_alpha = texture_color.w + bg_color_alpha_contribution;

    FragColor = vec4(frag_from_texture + frag_from_color, frag_alpha);
}
