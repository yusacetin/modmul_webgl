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
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlShader, WebGlProgram, console};

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
    widescreen: bool
}

#[wasm_bindgen]
pub struct Canvas {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
    shape: Shape,
    bg: Color,
    u_color_location: web_sys::WebGlUniformLocation,
}

// Struct implementations

#[wasm_bindgen]
impl Shape {
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color.r = r;
        self.color.g = g;
        self.color.b = b;
    }

    pub fn get_match_index(&self, i: u32) -> u32 {
        return (i * self.mul) % self.points;
    }

    pub fn get_point_pos(&self, i: u32) -> Point {
        let theta: f32 = (i as f32) * 2.0 * std::f32::consts::PI / (self.points as f32) + (std::f32::consts::PI / 2.0);
        let mut x: f32 = self.r * theta.cos();
        let mut y: f32 = self.r * theta.sin();
        
        if (self.widescreen) {
            x = x * (self.dimensions.y / self.dimensions.x);
        } else {
            y = y * (self.dimensions.x / self.dimensions.y);
        }

        return Point{x, y};
    }

    pub fn get_points_vector(&self) -> Vec<f32> {
        let mut pts: Vec<f32> = Vec::with_capacity(self.points as usize);
        for i in 0..self.points {
            let p: Point  =self.get_point_pos(i);
            pts.push(p.x);
            pts.push(p.y);
        }
        return pts;
    }

    pub fn get_lines_vector(&self) -> Vec<f32> {
        let mut lines: Vec<f32> = Vec::with_capacity((self.points * 4) as usize);
        for i in 0..self.points {
            let p0: Point = self.get_point_pos(i);
            let matchi: u32 = self.get_match_index(i);
            let p1: Point = self.get_point_pos(matchi);
            
            lines.push(p0.x);
            lines.push(p0.y);
            lines.push(p1.x);
            lines.push(p1.y);
        }
        return lines;
    }
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Canvas, JsValue> {
        // Get elements
        let window: web_sys::Window = web_sys::window().unwrap();
        let document: web_sys::Document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("webgl_canvas").unwrap().dyn_into::<HtmlCanvasElement>()?;
        let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

        // Compile vertex shader
        let vertex_shader_source: &'static str = r#"#version 300 es
            in vec2 position;
            in float size;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                gl_PointSize = size;
            }
        "#;
        let vertex_shader: WebGlShader = compile_shader(&gl, vertex_shader_source, WebGl2RenderingContext::VERTEX_SHADER).map_err(|e| JsValue::from_str(&e))?;

        // Compile fragment shader
        let fragment_shader_source: &'static str = r#"#version 300 es
            precision mediump float;
            uniform vec3 u_color;
            out vec4 outColor;
            void main() {
                outColor = vec4(u_color, 1.0);
            }
        "#;
        let fragment_shader: WebGlShader = compile_shader(&gl, fragment_shader_source, WebGl2RenderingContext::FRAGMENT_SHADER).map_err(|e| JsValue::from_str(&e))?;

        // Link WebGL program
        let program: WebGlProgram = link_program(&gl, &vertex_shader, &fragment_shader).map_err(|e| JsValue::from_str(&e))?;
        gl.use_program(Some(&program));

        // Save uniform color location so that we can change the foreground color later without having to recompile fragment shaders
        let u_color_location: web_sys::WebGlUniformLocation = gl.get_uniform_location(&program, "u_color").ok_or("Failed to find u_color uniform")?;

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
            widescreen: (canvas.width() >= canvas.height())
        };

        // Return self
        return Ok(Canvas {
            context: gl,
            program: program,
            shape,
            bg: Color {
                r: 24,
                g: 24,
                b: 24
            },
            u_color_location: u_color_location
        });
    }

    pub fn set_points(&mut self, value: u32) {
        self.shape.points = value;
    }

    pub fn set_multiplier(&mut self, value: u32) {
        self.shape.mul = value;
    }

    pub fn clear(&self) {
        let r: f32 = normalize_u8_to_1(self.bg.r);
        let g: f32 = normalize_u8_to_1(self.bg.g);
        let b: f32 = normalize_u8_to_1(self.bg.b);
        self.context.clear_color(r, g, b, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update_fg_color(&self) {
        self.context.use_program(Some(&self.program));
        let red: f32 = normalize_u8_to_1(self.shape.color.r);
        let green: f32 = normalize_u8_to_1(self.shape.color.g);
        let blue: f32 = normalize_u8_to_1(self.shape.color.b);
        self.context.uniform3f(Some(&self.u_color_location), red, green, blue);
    }

    pub fn draw(&self) {
        self.update_fg_color();
        self.draw_lines();
        self.draw_points();
    }

    pub fn draw_lines(&self) {
        // Get lines vector
        let lines_vector: Vec<f32> = self.shape.get_lines_vector();

        // Create buffer
        let buffer: web_sys::WebGlBuffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let vertex_array: js_sys::Float32Array = unsafe{js_sys::Float32Array::view(&lines_vector)};
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGl2RenderingContext::STATIC_DRAW
        );

        let pos_loc: u32 = self.context.get_attrib_location(&self.program, "position") as u32;
        self.context.vertex_attrib_pointer_with_i32(pos_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(pos_loc);

        self.context.draw_arrays(WebGl2RenderingContext::LINES, 0, (self.shape.points * 4) as i32);
    }

    pub fn draw_points(&self) {
        // Get position vector
        let pos_vector: Vec<f32> = self.shape.get_points_vector();

        // Upload position buffer
        let pos_buffer: web_sys::WebGlBuffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
        let pos_array: js_sys::Float32Array = unsafe {js_sys::Float32Array::view(&pos_vector)};
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &pos_array,
            WebGl2RenderingContext::STATIC_DRAW
        );
        let pos_attrib: u32 = self.context.get_attrib_location(&self.program, "position") as u32;
        self.context.vertex_attrib_pointer_with_i32(pos_attrib, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(pos_attrib);

        // Upload size buffer
        let size_value: f32 = self.shape.point_size;
        let size_vector: Vec<f32> = vec![size_value; self.shape.points as usize];
        let size_buffer: web_sys::WebGlBuffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&size_buffer));
        let size_array: js_sys::Float32Array = unsafe { js_sys::Float32Array::view(&size_vector) };
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &size_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        let size_attrib = self.context.get_attrib_location(&self.program, "size") as u32;
        self.context.vertex_attrib_pointer_with_i32(size_attrib, 1, WebGl2RenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(size_attrib);

        // Draw
        self.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, self.shape.points as i32);
    }
}

// Helper functions

fn compile_shader(gl: &WebGl2RenderingContext, source: &str, shader_type: u32) -> Result<WebGlShader, String> {
    let shader: WebGlShader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
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

    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}

pub fn normalize_u8_to_1(arg: u8) -> f32 {
    return (arg as f32) / 255.0;
}