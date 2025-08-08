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

#![allow(unused_parens)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlUniformLocation, WebGl2RenderingContext, WebGlShader, WebGlProgram, console};

// Struct declarations

#[wasm_bindgen]
pub struct Point {
    x: f32,
    y: f32
}

#[wasm_bindgen]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[wasm_bindgen]
pub struct Shape {
    points: u32,
    mul: u32, // multiplier
    r: f32,
    color: Color,
    pos: Point,
    point_size: f32,
    dimensions: Point,
    widescreen: bool,
    rotation: f32,
    outline_width: f32,
    outline_segments: i32
}

#[wasm_bindgen]
pub struct Canvas {
    context: WebGl2RenderingContext,
    point_program: WebGlProgram,
    line_program: WebGlProgram,
    outline_program: WebGlProgram,
    shape: Shape,
    bg: Color,
    u_point_color_location: web_sys::WebGlUniformLocation,
    u_line_color_location: web_sys::WebGlUniformLocation,
    u_outline_color_location: web_sys::WebGlUniformLocation,
    enable_outline: bool
}

// Struct implementations

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Canvas, JsValue> {
        // Get elements
        let window: web_sys::Window = web_sys::window().unwrap();
        let document: web_sys::Document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("webgl_canvas").unwrap().dyn_into::<HtmlCanvasElement>()?;
        let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

        // Compile point shader
        let point_shader_src: &str = include_str!("point_shader.vert");
        let point_shader: WebGlShader = compile_shader(&gl, point_shader_src, WebGl2RenderingContext::VERTEX_SHADER).map_err(|e: String| JsValue::from_str(&e))?;

        // Compile line shader
        let line_shader_src: &str = include_str!("line_shader.vert");
        let line_shader: WebGlShader = compile_shader(&gl, line_shader_src, WebGl2RenderingContext::VERTEX_SHADER).map_err(|e: String| JsValue::from_str(&e))?;

        // Compile outline shader
        let outline_shader_src: &str = include_str!("outline_shader.vert");
        let outline_shader: WebGlShader = compile_shader(&gl, outline_shader_src, WebGl2RenderingContext::VERTEX_SHADER).map_err(|e: String| JsValue::from_str(&e))?;

        // Compile color shader
        let color_shader_src: &str = include_str!("color_shader.frag");
        let color_shader: WebGlShader = compile_shader(&gl, color_shader_src, WebGl2RenderingContext::FRAGMENT_SHADER).map_err(|e: String| JsValue::from_str(&e))?;

        // Link WebGL programs
        let point_program: WebGlProgram = link_program(&gl, &point_shader, &color_shader).map_err(|e: String| JsValue::from_str(&e))?;
        let line_program: WebGlProgram = link_program(&gl, &line_shader, &color_shader).map_err(|e: String| JsValue::from_str(&e))?;
        let outline_program: WebGlProgram = link_program(&gl, &outline_shader, &color_shader).map_err(|e: String| JsValue::from_str(&e))?;
        gl.use_program(Some(&point_program)); // call from drawing function instead

        // Save uniform color location so that we can change the foreground color later easily
        let u_point_color_location: web_sys::WebGlUniformLocation = gl.get_uniform_location(&point_program, "u_color").ok_or("Failed to find u_color uniform")?;
        let u_line_color_location: web_sys::WebGlUniformLocation = gl.get_uniform_location(&line_program, "u_color").ok_or("Failed to find u_color uniform")?;
        let u_outline_color_location: web_sys::WebGlUniformLocation = gl.get_uniform_location(&outline_program, "u_color").ok_or("Failed to find u_color uniform")?;

        // Adjust to window size
        let dpr: f64 = window.device_pixel_ratio();
        let w: u32 = (window.inner_width()?.as_f64().unwrap() * dpr) as u32;
        let h: u32 = (window.inner_height()?.as_f64().unwrap() * dpr) as u32;
        canvas.set_width(w);
        canvas.set_height(h);
        gl.viewport(0, 0, w as i32, h as i32);

        // Create shape
        let shape: Shape = Shape {
            points: 500,
            mul: 72,
            r: 0.92,
            color: Color {
                r: 250,
                g: 250,
                b: 250
            },
            pos: Point {
                x: 0.0,
                y: 0.0
            },
            point_size: 2.0,
            dimensions: Point {
                x: canvas.width() as f32,
                y: canvas.height() as f32
            },
            widescreen: (canvas.width() >= canvas.height()),
            rotation: 0.0,
            outline_width: 0.005,
            outline_segments: 1440
        };

        // Return self
        return Ok(Canvas {
            context: gl,
            point_program: point_program,
            line_program: line_program,
            outline_program: outline_program,
            shape,
            bg: Color {
                r: 24,
                g: 24,
                b: 24
            },
            u_point_color_location: u_point_color_location,
            u_line_color_location: u_line_color_location,
            u_outline_color_location: u_outline_color_location,
            enable_outline: false
        });
    }

    // Meant to be called when window gets resized
    pub fn adjust_view(&mut self) -> Result<bool, JsValue> {
        // Get elements
        let window: web_sys::Window = web_sys::window().unwrap();
        let document: web_sys::Document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("webgl_canvas").unwrap().dyn_into::<HtmlCanvasElement>()?;
        let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

        // Adjust to window size
        let dpr: f64 = window.device_pixel_ratio();
        let w: u32 = (window.inner_width()?.as_f64().unwrap() * dpr) as u32;
        let h: u32 = (window.inner_height()?.as_f64().unwrap() * dpr) as u32;
        canvas.set_width(w);
        canvas.set_height(h);
        gl.viewport(0, 0, w as i32, h as i32);

        // Update object properties according to new demensions
        self.shape.dimensions.x = canvas.width() as f32;
        self.shape.dimensions.y = canvas.height() as f32;
        self.shape.widescreen = (canvas.width() >= canvas.height());

        // Draw again
        self.clear();
        self.draw();

        return Ok(true);
    }

    pub fn reset(&mut self) {
        self.shape.r = 0.92;
        self.shape.pos = Point {x: 0.0, y: 0.0};
        self.shape.rotation = 0.0;

        // Draw again
        self.clear();
        self.draw();
    }

    pub fn set_points(&mut self, value: u32) {
        self.shape.points = value;
        if (value >  1440) {
            self.shape.outline_segments = value as i32;
        } else {
            self.shape.outline_segments = 1440;
        }
    }

    pub fn set_multiplier(&mut self, value: u32) {
        self.shape.mul = value;
    }

    pub fn set_rotation(&mut self, deg: f32) {
        // first convert degrees to radians
        let rad: f32 = deg * std::f32::consts::PI / 180.0;
        self.shape.rotation = rad;

        // Draw again
        self.clear();
        self.draw();
    }

    pub fn clear(&self) {
        let r: f32 = normalize_u8_to_1(self.bg.r);
        let g: f32 = normalize_u8_to_1(self.bg.g);
        let b: f32 = normalize_u8_to_1(self.bg.b);
        self.context.clear_color(r, g, b, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update_fg_color(&self) {
        let red: f32 = normalize_u8_to_1(self.shape.color.r);
        let green: f32 = normalize_u8_to_1(self.shape.color.g);
        let blue: f32 = normalize_u8_to_1(self.shape.color.b);

        self.context.use_program(Some(&self.point_program));
        self.context.uniform3f(Some(&self.u_point_color_location), red, green, blue);

        self.context.use_program(Some(&self.line_program));
        self.context.uniform3f(Some(&self.u_line_color_location), red, green, blue);

        self.context.use_program(Some(&self.outline_program));
        self.context.uniform3f(Some(&self.u_outline_color_location), red, green, blue);
    }

    pub fn draw(&self) {
        self.update_fg_color();
        if (self.enable_outline) {
            self.draw_outline();
        }
        self.draw_lines();
        self.draw_points();
    }

    pub fn draw_outline(&self) {
        self.context.use_program(Some(&self.outline_program));
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.outline_program, "u_segments").expect("Error")), self.shape.outline_segments as f32);
        self.context.uniform2f(Some(&self.context.get_uniform_location(&self.outline_program, "u_center").expect("Error")), self.shape.pos.x, self.shape.pos.y);

        let mut x_norm: f32 = 1.0;
        let mut y_norm: f32 = 1.0;
        if (self.shape.widescreen) {
            x_norm = self.shape.dimensions.y / self.shape.dimensions.x;
        } else {
            y_norm = self.shape.dimensions.x / self.shape.dimensions.y;
        }
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.outline_program, "u_x_norm").expect("Error")), x_norm);
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.outline_program, "u_y_norm").expect("Error")), y_norm);


        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.outline_program, "u_radius").expect("Error")), self.shape.r + self.shape.outline_width / 2.0);
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, self.shape.outline_segments + 2);
        
        let bg_red: f32 = normalize_u8_to_1(24);
        let bg_green: f32 = normalize_u8_to_1(24);
        let bg_blue: f32 = normalize_u8_to_1(24);
        self.context.uniform3f(Some(&self.u_outline_color_location), bg_red, bg_green, bg_blue);

        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.outline_program, "u_radius").expect("Error")), self.shape.r - self.shape.outline_width / 2.0);
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, self.shape.outline_segments + 2);
    }

    pub fn draw_lines(&self) {
        self.context.use_program(Some(&self.line_program));
        self.context.uniform1i(Some(&self.context.get_uniform_location(&self.line_program, "u_points").expect("Error")), self.shape.points as i32);
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.line_program, "u_radius").expect("Error")), self.shape.r);
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.line_program, "u_rotation").expect("Error")), self.shape.rotation);
        self.context.uniform2f(Some(&self.context.get_uniform_location(&self.line_program, "u_position").expect("Error")), self.shape.pos.x, self.shape.pos.y);
        self.context.uniform2f(Some(&self.context.get_uniform_location(&self.line_program, "u_dimensions").expect("Error")), self.shape.dimensions.x, self.shape.dimensions.y);
        self.context.uniform1i(Some(&self.context.get_uniform_location(&self.line_program, "u_widescreen").expect("Error")), if self.shape.widescreen { 1 } else { 0 });
        self.context.uniform1i(Some(&self.context.get_uniform_location(&self.line_program, "u_multiplier").expect("Error")), self.shape.mul as i32);
        self.context.draw_arrays(WebGl2RenderingContext::LINES, 0, (self.shape.points * 2) as i32);
    }

    pub fn draw_points(&self) {
        self.context.use_program(Some(&self.point_program));
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.point_program, "u_points").expect("Error")), self.shape.points as f32);
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.point_program, "u_radius").expect("Error")), self.shape.r);
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.point_program, "u_rotation").expect("Error")), self.shape.rotation);
        self.context.uniform2f(Some(&self.context.get_uniform_location(&self.point_program, "u_position").expect("Error")), self.shape.pos.x, self.shape.pos.y);
        self.context.uniform2f(Some(&self.context.get_uniform_location(&self.point_program, "u_dimensions").expect("Error")), self.shape.dimensions.x, self.shape.dimensions.y);
        self.context.uniform1i(Some(&self.context.get_uniform_location(&self.point_program, "u_widescreen").expect("Error")), if self.shape.widescreen { 1 } else { 0 });
        self.context.uniform1f(Some(&self.context.get_uniform_location(&self.point_program, "u_point_size").expect("Error")), self.shape.point_size);
        self.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, self.shape.points as i32);
    }

    pub fn get_r(&self) -> f32 {
        return self.shape.r;
    }

    pub fn set_enable_outline(&mut self, value: bool) {
        self.enable_outline = value;
    }

    // Increase radius and adjust the new location of the center so that
    // the zoom effect appears as originating from the cursor location
    // mx = mouse x, my = mouse y
    pub fn add_to_r(&mut self, val: f32, mx: f32, my: f32) {
        let prev_r: f32 = self.shape.r; // previous value of r will be needed in calculations
        self.shape.r += val; // calculate new r

        // We must denormalize shape position because position is saved in normalized coordinates
        // Note: normalization refers to widescreen or narrowscreen adjustment because
        // WebGL assumed a value of 2.0 equals both the height and width of the screen
        let mut denorm_x: f32 = self.shape.pos.x;
        let mut denorm_y: f32 = self.shape.pos.y;
        if (self.shape.widescreen) {
            denorm_x = self.shape.pos.x * self.shape.dimensions.y / self.shape.dimensions.x;
        } else {
            denorm_y = self.shape.pos.y * self.shape.dimensions.x / self.shape.dimensions.y;
        }

        let prev_dx: f32 = (mx - denorm_x).abs();
        let prev_dy: f32 = (my - denorm_y).abs();

        let dx: f32 = self.shape.r * prev_dx / prev_r - prev_dx;
        let dy: f32 = self.shape.r * prev_dy / prev_r - prev_dy;

        if (mx > denorm_x) {
            denorm_x -= dx;
        } else {
            denorm_x += dx;
        }

        if (my > denorm_y) {
            denorm_y -= dy;
        } else {
            denorm_y += dy;
        }

        if (self.shape.widescreen) {
            self.shape.pos.x = denorm_x * self.shape.dimensions.x / self.shape.dimensions.y;
            self.shape.pos.y = denorm_y;
        } else {
            self.shape.pos.x = denorm_x;
            self.shape.pos.y = denorm_y * self.shape.dimensions.y / self.shape.dimensions.x;
        }
        
        // Draw again
        self.clear();
        self.draw();
    }

    pub fn move_shape(&mut self, dx: f32, dy: f32) {
        self.shape.pos.x += dx;
        self.shape.pos.y += dy;
        self.clear();
        self.draw();
    }
}

// Helper functions

fn compile_shader(gl: &WebGl2RenderingContext, source: &str, shader_type: u32) -> Result<WebGlShader, String> {
    let shader: WebGlShader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if (gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false)) {
        return Ok(shader);
    } else {
        return Err(gl.get_shader_info_log(&shader).unwrap_or_default());
    }
}

fn link_program(gl: &WebGl2RenderingContext, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program: WebGlProgram = gl.create_program().ok_or("Failed to create program")?;
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if (gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false)) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}

pub fn normalize_u8_to_1(arg: u8) -> f32 {
    return (arg as f32) / 255.0;
}