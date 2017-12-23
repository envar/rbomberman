#version 150 core

noperspective in vec3 g_Dist;

out vec4 Target0;

// The mesh line settings
uniform float u_LineWidth;
uniform vec4 u_LineColor;

vec4 color = vec4(0.0, 0.5, 1.0, 1.0);

void main() {
	// find the smallest distance
    float d = min(g_Dist.x, min(g_Dist.y, g_Dist.z));

    // Determine the mix factor with line color
    float mixVal = smoothstep(u_LineWidth - 1, u_LineWidth + 1, d);

    // Mix the surface color with the line color
	Target0 = mix(u_LineColor, color, mixVal);
}
