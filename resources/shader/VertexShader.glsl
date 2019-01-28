#version 430 core

layout(location = 0) in vec3 vertex_pos;
layout(location = 1) in vec3 vertex_uv;
layout(location = 2) in vec3 vertex_normal;

out vec3 uv;
out vec3 normal;

uniform mat4 MVP;

void main() {
  gl_Position = MVP * vec4(vertex_pos, 1);
  uv = vertex_uv;
  normal = vertex_normal;
}
