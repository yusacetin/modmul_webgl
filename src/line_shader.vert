#version 300 es
precision highp float;

uniform int u_points;
uniform float u_radius;
uniform float u_rotation;
uniform vec2 u_position;
uniform vec2 u_dimensions;
uniform bool u_widescreen;
uniform int u_multiplier;

void main() {
    float pi = 3.1415926535897932384626;
    
    int i = gl_VertexID;
    bool is_dst = ((i % 2) == 1);
    int line_i = i / 2;
    if (is_dst) {
        line_i = (line_i * u_multiplier) %  u_points;
    }
    float line_i_float = float(line_i);
    float points_float = float(u_points);
    
    float theta =  line_i_float * 2.0 * pi / points_float + u_rotation + (pi / 2.0);

    float x = -1.0 * cos(theta) * u_radius + u_position.x;
    float y = sin(theta) * u_radius + u_position.y;

    if (u_widescreen) {
        x *= u_dimensions.y / u_dimensions.x;
    } else {
        y *= u_dimensions.x / u_dimensions.y;
    }

    gl_Position = vec4(x, y, 0.0, 1.0);
}
