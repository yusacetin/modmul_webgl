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
    canvas.clear();
    canvas.draw();

    // change number of points by scrolling over the input
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

    // change multiplier by scrolling over the input
    let mul_input = document.getElementById("mul_input");
    mul_input.addEventListener("change", redraw);
    mul_input.addEventListener("wheel", function(e) {
        e.preventDefault();
        const step = parseFloat(mul_input.step) || 1;
        const current = parseFloat(mul_input.value) || 0;
        if (e.deltaY < 0) {
            mul_input.value = current + step;
        } else {
            mul_input.value = current - step;
        }
        redraw();
    });

    // zoom in and out by scrolling over the canvas
    let webgl_canvas = document.getElementById("webgl_canvas");
    webgl_canvas.addEventListener("wheel", function(e) {
        e.preventDefault();
        let step = canvas.get_r() / 5.0;
        if (e.deltaY < 0) {step *= -1;}

        let normalized_x = (e.clientX - window.innerWidth/2) / (window.innerWidth / 2);
        let normalized_y = (window.innerHeight/2 - e.clientY) / (window.innerHeight / 2);

        canvas.add_to_r(step, normalized_x, normalized_y);
    });

    let reset_view_button = document.getElementById("reset_view_button");
    reset_view_button.addEventListener("click", () => {
        canvas.reset();
    });

    // adjust to new window size when resized
    window.addEventListener("resize", () => {
        canvas.adjust_view();
    });

    console.log("Loading complete");
}

function redraw() {
    canvas.set_points(document.getElementById("points_input").value);
    canvas.set_multiplier(document.getElementById("mul_input").value);
    canvas.clear();
    canvas.draw();
}

// Movement state logic

let dragging = false;
let last_x = 0.0;
let last_y = 0.0;

let webgl_canvas = document.getElementById("webgl_canvas");

// When mouse button is pressed, we start the drag state
webgl_canvas.addEventListener("mousedown", (e) => {
    if (e.button !== 0) {
        // Only accept left click
        return;
    }
    dragging = true;
    last_x = e.clientX;
    last_y = e.clientY;
    webgl_canvas.style.cursor = "move";
});

// When mouse button is released, we end the drag state
webgl_canvas.addEventListener("mouseup", (e) => {
    if (e.button !== 0) {
        return;
    }
    dragging = false;
    webgl_canvas.style.cursor = "default";
});

// When cursor leaves the screen, we should end the drag state to not cause confusion
webgl_canvas.addEventListener("mouseleave", (e) => {
    if (e.button !== 0) {
        return;
    }
    dragging = false;
    webgl_canvas.style.cursor = "default";
}); 

// If in drag mode, move the canvas
// If not, don't do anything
webgl_canvas.addEventListener("mousemove", (e) => {
    if (!dragging) {
        return;
    }

    let w = window.innerWidth;
    let h = window.innerHeight;

    let dx = (e.clientX - last_x) * 2;
    let dy = (last_y - e.clientY) * 2;
    if (w >= h) {
        dx /= h;
        dy /= h;
    } else {
        dx /= w;
        dy /= w;
    }

    last_x = e.clientX;
    last_y = e.clientY;

    canvas.move_shape(dx, dy);
});

run();