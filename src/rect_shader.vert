// Work in progress, not used yet

#version 300 es

uniform float u_points;
uniform float u_radius;
uniform float u_rotation;
uniform vec2 u_position;
uniform vec2 u_dimensions;
uniform bool u_widescreen;
uniform float u_multiplier;

void main() {
    float pi = 3.1415926535897932384626;
    float base_i = float(gl_VertexID) / 2.0;
    bool is_start = mod(float(gl_VertexID), 2.0) < 0.5;

    float i = is_start ? base_i : mod(base_i * u_multiplier, u_points);
    float theta = i * 2.0 * pi / u_points + (pi / 2.0) - u_rotation;
}