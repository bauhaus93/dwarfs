#version 330 core

in vec3 uv;
in vec3 normal;
out vec3 color;

uniform sampler2DArray textureArray;

void main() {
    if (uv[0] < 0.01 || uv[1] < 0.01 || uv[0] > 0.99 || uv[1] > 0.99) {
        color = vec3(0.0, 1.0, 0.0);
    } else {
        color = texture(textureArray, uv).rgb;
    }
    if (abs(normal[2] - 1.0) > 0.1) {
        color *= 0.66;
    }
}
