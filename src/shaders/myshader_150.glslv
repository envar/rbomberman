#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;

uniform mat4 u_Transform;

out vec2 v_Uv;

void main() {
    gl_Position = u_Transform * vec4(a_Pos, 0.0, 1.0);
    v_Uv = a_Uv;
}