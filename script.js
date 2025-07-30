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

import init, {Canvas} from "./pkg/modmul_webgl.js"

let canvas;

async function run() {
    await init();

    canvas = new Canvas();
    //resize_webgl_canvas();
    canvas.clear();
    canvas.draw();

    let points_input = document.getElementById("points_input");
    points_input.addEventListener("change", redraw);
    points_input.addEventListener("wheel", function(e) {
        e.preventDefault(); // prevent page scrolling
        const step = parseFloat(points_input.step) || 1;
        const current = parseFloat(points_input.value) || 0;
        if (e.deltaY < 0) {
            if (current + step >= 0) {
                points_input.value = current + step;
            }
        } else {
            if (current - step >= 0) {
                points_input.value = current - step;
            }
        }
        redraw();
    });

    let mul_input = document.getElementById("mul_input");
    mul_input.addEventListener("change", redraw);
    mul_input.addEventListener("wheel", function(e) {
        e.preventDefault(); // prevent page scrolling
        const step = parseFloat(mul_input.step) || 1;
        const current = parseFloat(mul_input.value) || 0;
        if (e.deltaY < 0) {
            mul_input.value = current + step;
        } else {
            mul_input.value = current - step;
        }
        redraw();
    });
    
    console.log("Loading complete");
}

function resize_webgl_canvas() {
    const elem = document.getElementById("webgl_canvas");

    const width = window.innerWidth;
    const height = window.innerHeight;
    elem.style.width = width + "px";
    elem.style.height = height + "px";
    console.log("height: " + height);

    const dpr = window.devicePixelRatio || 1;
    elem.width = width * dpr;
    elem.height = height * dpr;

    const gl = elem.getContext("webgl2");
    gl.viewport(0, 0, elem.width, elem.height);
}

function redraw() {
    canvas.set_points(document.getElementById("points_input").value);
    canvas.set_multiplier(document.getElementById("mul_input").value);
    canvas.clear();
    canvas.draw();
}

run();