#version 330

in vec2 textureCoords;

out vec4 FragColor;

uniform vec4 color;
uniform sampler2D texture0;
uniform vec4 drawBounds; // x = top, y = right, z = bottom, w = left

void main() {
    // check draw bounds
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

    // calculate frag color
    vec4 texture_color = texture(texture0, textureCoords);
    float bg_color_alpha_contribution = (1.0 - texture_color.w) * color.w;

    vec3 frag_from_texture = texture_color.xyz * texture_color.w;
    vec3 frag_from_color = color.xyz * bg_color_alpha_contribution;
    float frag_alpha = texture_color.w + bg_color_alpha_contribution;

    FragColor = vec4(frag_from_texture + frag_from_color, frag_alpha);
}
