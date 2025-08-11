#version 300 es
precision highp float;

/*
This file is part of Modular Multiplication WebGL.

Modular Multiplication WebGL is free software: you can redistribute it
and/or modify it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at your option)
any later version.

Modular Multiplication WebGL is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Modular 
Multiplication WebGL. If not, see <https://www.gnu.org/licenses/>.
*/

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
