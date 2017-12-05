#version 150 core

layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

out vec4 

void main() {
    // TODO: compute distances to pass into fragment shader

    gl_Position = gl_in[0].gl_Position;
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
    EmitVertex();

    gl_Position = gl_in[2].gl_Position;
    EmitVertex();

    EndPrimitive();
}
