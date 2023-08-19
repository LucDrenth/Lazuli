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

    float alpha = texture(texture0, textureCoords).r;
    FragColor = vec4(color, alpha);
}
