#version 300 es
precision highp float;

uniform int u_points;
uniform float u_radius;
uniform float u_rotation;
uniform vec2 u_position;
uniform vec2 u_dimensions;
uniform bool u_widescreen;
uniform int u_multiplier;
uniform float u_rectw;

void main() {
    float pi = 3.1415926535897932384626;
    int vi = gl_VertexID;

    // Calculate necessary parameters
    int src_i = vi / 6;
    int dst_i = (src_i * u_multiplier) %  u_points;
    int local_vi = vi % 6; // index of vertex within current rectangle

    float fsrc_i = float(src_i);
    float fdst_i = float(dst_i);
    float fpoints = float(u_points);
    float src_theta = fsrc_i * 2.0 * pi / fpoints + u_rotation + (pi / 2.0);
    float dst_theta = fdst_i * 2.0 * pi / fpoints + u_rotation + (pi / 2.0);

    // Calculate source vertex position
    float src_x = -1.0 * cos(src_theta) * u_radius + u_position.x;
    float src_y = sin(src_theta) * u_radius + u_position.y;

    // Calculate destination vertex position
    float dst_x = -1.0 * cos(dst_theta) * u_radius + u_position.x;
    float dst_y = sin(dst_theta) * u_radius + u_position.y;

    // Calculate center of rectangle to be drawn
    float rect_x = (src_x + dst_x) / 2.0;
    float rect_y = (src_y + dst_y) / 2.0;

    // Calculate sin and cos of rotation
    // We don't need to use trigonometric functions because we can calculate
    // the proportions of the lengths of the triangle
    float b = src_x - dst_x;
    float h = src_y - dst_y;
    float hypotenuse = sqrt(b*b + h*h);
    float cosrot = 1.0; // for if hypotenuse == 0
    float sinrot = 0.0; // for if hypotenuse == 0
    if (hypotenuse > 0.0) {
        cosrot = b / hypotenuse;
        sinrot = h / hypotenuse;
    }
    float cos90_minus_rot = sinrot; // cos(pi/2 - x) = sin(x)
    float sin90_minus_rot = cosrot; // sin(pi/2 - x) = cos(x)

    // Now that we know the center and trigonometric values of the rectangle we can draw it
    vec2 res = vec2(0.0, 0.0);
    
    // 0 for top left of first triangle
    if (local_vi == 0) {
        float x_begin = rect_x - (cos90_minus_rot * u_rectw / 2.0);
        float y_begin = rect_y + (sin90_minus_rot * u_rectw / 2.0);

        float dx = hypotenuse / 2.0 * cosrot;
        float dy = hypotenuse / 2.0 * sinrot;

        res.x = x_begin - dx;
        res.y = y_begin - dy;
    }

    // 1 and 5 for bottom left of first and second triangle
    else if (local_vi == 1 || local_vi == 5) {
        float x_begin = rect_x + (cos90_minus_rot * u_rectw / 2.0);
        float y_begin = rect_y - (sin90_minus_rot * u_rectw / 2.0);

        float dx = hypotenuse / 2.0 * cosrot;
        float dy = hypotenuse / 2.0 * sinrot;

        res.x = x_begin - dx;
        res.y = y_begin - dy;
    }
    
    // 2 and 3 for top right of first and second triangle
    else if (local_vi == 2 || local_vi == 3) {
        float x_begin = rect_x - (cos90_minus_rot * u_rectw / 2.0);
        float y_begin = rect_y + (sin90_minus_rot * u_rectw / 2.0);

        float dx = hypotenuse / 2.0 * cosrot;
        float dy = hypotenuse / 2.0 * sinrot;
        
        res.x = x_begin + dx;
        res.y = y_begin + dy;
    }

    // 4 for bottom right of second triangle
    else if (local_vi == 4) {
        float x_begin = rect_x + (cos90_minus_rot * u_rectw / 2.0);
        float y_begin = rect_y - (sin90_minus_rot * u_rectw / 2.0);

        float dx = hypotenuse / 2.0 * cosrot;
        float dy = hypotenuse / 2.0 * sinrot;
        
        res.x = x_begin + dx;
        res.y = y_begin + dy;
    }

    // Normalize for non-square viewport
    if (u_widescreen) {
        res.x *= u_dimensions.y / u_dimensions.x;
    } else {
        res.y *= u_dimensions.x / u_dimensions.y;
    }

    gl_Position = vec4(res, 0.0, 1.0);
}