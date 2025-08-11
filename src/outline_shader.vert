#version 300 es
precision mediump float;

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

uniform float u_segments;
uniform float u_radius;
uniform float u_x_norm;
uniform float u_y_norm;
uniform vec2 u_center;

void main() {
    float pi = 3.1415926535897932384626;

    if (gl_VertexID == 0) {
        // Center of fan
        gl_Position = vec4(u_center.x * u_x_norm, u_center.y * u_y_norm, 0.0, 1.0);
    } else {
        // Perimeter vertex
        float segment = float(gl_VertexID - 1);
        float theta = (segment / u_segments) * 2.0 * pi;
        float x = (u_radius * cos(theta) + u_center.x) * u_x_norm;
        float y = (u_radius * sin(theta) + u_center.y) * u_y_norm;
        gl_Position = vec4(x, y, 0.0, 1.0);
    }
}