#version 330 core

in vec3 uv;
in vec3 normal;
out vec3 color;

uniform sampler2DArray textureArray;

void main() {
    if (uv[0] < 0.02 || uv[1] < 0.02 || uv[0] > 0.98 || uv[1] > 0.98) {
        color = vec3(1, 1, 0);
    } else {
        color = texture(textureArray, uv).rgb;
    }
    if (abs(normal[2] - 1.0) > 0.1) {
        color *= 0.66;
    }
}
