#version 300 es
precision highp float;

uniform float u_points;
uniform float u_radius;
uniform float u_rotation;
uniform vec2 u_position;
uniform vec2 u_dimensions;
uniform bool u_widescreen;
uniform float u_point_size;

void main() {
    float pi = 3.1415926535897932384626;
    float i = float(gl_VertexID);
    float theta =  i * 2.0 * pi / u_points + u_rotation + (pi / 2.0);

    float x = -1.0 * cos(theta) * u_radius + u_position.x;
    float y = sin(theta) * u_radius + u_position.y;

    // Aspect-ratio correction
    if (u_widescreen) {
        x *= u_dimensions.y / u_dimensions.x;
    } else {
        y *= u_dimensions.x / u_dimensions.y;
    }

    gl_Position = vec4(x, y, 0.0, 1.0);
    gl_PointSize = u_point_size;
}
