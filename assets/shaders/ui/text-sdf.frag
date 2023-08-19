#version 330

out vec4 FragColor;

in vec2 textureCoords;

uniform sampler2D texture0;
uniform vec3 color;

// x = top, y = right, z = bottom, w = left
uniform vec4 drawBounds;

void main() {
    float boundTop = drawBounds.x;
    float boundRight = drawBounds.y;
    float boundBottom = drawBounds.z;
    float boundLeft = drawBounds.w;
    
    if (gl_FragCoord.y > boundTop || 
        gl_FragCoord.y < boundBottom || 
        gl_FragCoord.x < boundLeft || 
        gl_FragCoord.x > boundRight
    ) {
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
