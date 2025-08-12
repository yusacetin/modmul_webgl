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
    canvas.draw();

    // change number of points
    {
        // by input box
        let points_input = document.getElementById("points_input");
        points_input.addEventListener("input", (e) => {
            if (isNaN(points_input.value)) {
                return;
            }

            let val = points_input.value;
            if (val < 1) {
                val = 1;
            }
            canvas.set_points(val);
        });

        // by mouse wheel
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
            canvas.set_points(points_input.value);
        });
    }

    // change multiplier 
    {
        // by input box
        let mul_input = document.getElementById("mul_input");
        mul_input.addEventListener("input", (e) => {
            if (isNaN(mul_input.value)) {
                return;
            }

            let val = mul_input.value;
            if (val < 1) {
                val = 0;
            }
            canvas.set_multiplier(val);
        });

        // by mouse wheel
        mul_input.addEventListener("wheel", function(e) {
            e.preventDefault();
            const step = parseFloat(mul_input.step) || 1;
            const current = parseFloat(mul_input.value) || 0;
            if (e.deltaY < 0) {
                mul_input.value = current + step;
            } else {
                if (current - step >= parseInt(mul_input.getAttribute("min"))) {
                    mul_input.value = current - step;
                }
            }
            canvas.set_multiplier(mul_input.value);
        });
    }

    // zoom in and out by scrolling over the canvas
    {
        let webgl_canvas = document.getElementById("webgl_canvas");
        webgl_canvas.addEventListener("wheel", function(e) {
            e.preventDefault();
            let step = canvas.get_r() / 5.0;
            if (e.deltaY >= 0) {step *= -1;}

            let normalized_x = (e.clientX - window.innerWidth/2) / (window.innerWidth / 2);
            let normalized_y = (window.innerHeight/2 - e.clientY) / (window.innerHeight / 2);

            canvas.add_to_r(step, normalized_x, normalized_y);
        });
    }
    
    // reset view
    {
        let reset_view_button = document.getElementById("reset_view_button");
        reset_view_button.addEventListener("click", () => {
            document.getElementById("rotation_slider").value = 0;
            document.getElementById("rotation_label").textContent = 0;
            canvas.reset();
        });
    }
    
    // change rotation
    {
        let rotation_slider = document.getElementById("rotation_slider");
        rotation_slider.addEventListener("input", function() {
            document.getElementById("rotation_label").textContent = this.value;
            canvas.set_rotation(this.value);
        });
    }
    
    // set enable draw outline
    {
        let draw_outline_cb = document.getElementById("draw_outline_cb");
        draw_outline_cb.addEventListener("change", (e) => {
            let val = draw_outline_cb.checked;
            canvas.set_enable_outline(val);
        });
    }

    // change line width
    {
        // by mouse wheel
        let line_width_input = document.getElementById("line_width");
        line_width_input.addEventListener("wheel", (e) => {
            e.preventDefault(); // prevent page scrolling
            if (line_width_input.getAttribute("disabled")) {
                return;
            }
            const step = parseFloat(line_width_input.step) || 1;
            const current = parseFloat(line_width_input.value) || 0;
            if (e.deltaY < 0) {
                if (current + step >= 0) {
                    line_width_input.value = current + step;
                }
            } else {
                if (current - step >= 0) {
                    line_width_input.value = current - step;
                }
            }
            
            let val = line_width_input.value;
            if (val <= 0) {
                val = 0.0001;
                line_width_input.value = 1;
            } else {
                val /= 10000.0;
            }
            canvas.set_rect_width(val);
        });

        // by input box
        line_width_input.addEventListener("input", (e) => {
            if (isNaN(line_width_input.value)) {
                return;
            }

            let val = line_width_input.value;
            if (val <= 0) {
                val = 0.0001;
            } else {
                val /= 10000.0;
            }
            canvas.set_rect_width(val);
        });
    }

    // change shader
    {
        // to lines
        let line_width_input = document.getElementById("line_width");
        let lines_rb = document.getElementById("lines_rb");
        lines_rb.addEventListener("change", (e) => {
            let val = !lines_rb.checked;
            canvas.set_use_rects(val);

            if (val) {
                line_width_input.removeAttribute("disabled");
            } else {
                line_width_input.setAttribute("disabled", true);
            }
        });

        // to rects
        let rects_rb = document.getElementById("rects_rb");
        rects_rb.addEventListener("change", (e) => {
            let val = rects_rb.checked;
            canvas.set_use_rects(val);

            if (val) {
                line_width_input.removeAttribute("disabled");
            } else {
                line_width_input.setAttribute("disabled", true);
            }
        });
    }

    // panel show/hide buttons
    {
        let hide_button = document.getElementById("hide_button");
        let show_button = document.getElementById("show_button");
        let panel = document.getElementById("control_panel");
        hide_button.addEventListener("click", (e) => {
            show_button.removeAttribute("style");
            panel.setAttribute("style", "display:none;")
        });
        show_button.addEventListener("click", (e) => {
            show_button.setAttribute("style", "display:none;");
            panel.removeAttribute("style");
        });
    }

    // adjust to new window size when resized
    {
        window.addEventListener("resize", () => {
            canvas.adjust_view();
        });
    }

    // Movement by mouse drag
    {
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
            if (e.relatedTarget && e.relatedTarget.id === "control_panel") {
                return;
            }
            if (e.relatedTarget && e.relatedTarget.id === "show_button") {
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
    }

    let touch_zooming = false; // cancel touch moving if zooming

    // Touchscreen move
    {
        let webgl_canvas = document.getElementById("webgl_canvas");

        let dragging = false;
        let prev_x = 0.0;
        let prev_y = 0.0;

        // On first touch, begin dragging mode
        webgl_canvas.addEventListener("touchstart", function(e) {
            if (e.touches.length !== 1) {
                return;
            }

            dragging = true;
            prev_x = e.touches[0].clientX;
            prev_y = e.touches[0].clientY;
        });

        // Exit dragging mode when touch event ends
        webgl_canvas.addEventListener("touchend", function(e) {
            dragging = false;
        });

        webgl_canvas.addEventListener("touchcancel", function(e) {
            dragging = false;
        });

        // Handle drag event if in dragging mode
        webgl_canvas.addEventListener("touchmove", function(e) {
            e.preventDefault();
            if (!dragging) {
                return;
            }
            if (touch_zooming) {
                return;
            }

            const cur_x = e.touches[0].clientX;
            const cur_y = e.touches[0].clientY;

            let w = window.innerWidth;
            let h = window.innerHeight;

            let dx = (cur_x - prev_x) * 2;
            let dy = (prev_y - cur_y) * 2;
            if (w >= h) {
                dx /= h;
                dy /= h;
            } else {
                dx /= w;
                dy /= w;
            }

            prev_x = cur_x;
            prev_y = cur_y;

            canvas.move_shape(dx, dy);
        });
    }

    // Touchscreen zoom
    {
        // helper function
        function get_distance(touch0, touch1) {
            const dx = touch0.clientX - touch1.clientX;
            const dy = touch0.clientY - touch1.clientY;
            return Math.sqrt(dx*dx + dy*dy);
        }

        let webgl_canvas = document.getElementById("webgl_canvas");

        let prev_dist = 0.0;

        webgl_canvas.addEventListener("touchstart", function(e) {
            if (e.touches.length === 2) {
                prev_dist = get_distance(e.touches[0], e.touches[1]);
            }
        });

        webgl_canvas.addEventListener("touchmove", function(e) {
            if (e.touches.length !== 2) {
                return;
            }

            e.preventDefault();
            touch_zooming = true;
            const cur_r = canvas.get_r();
            const step_coef = 0.005 * cur_r;

            // Calculate difference in current and previous distance
            const cur_dist = get_distance(e.touches[0], e.touches[1]);
            const ddist_raw = cur_dist - prev_dist;
            const ddist = ddist_raw * step_coef;

            // Calculate center of pinch gesture
            let center_x = (e.touches[0].clientX + e.touches[1].clientX) / 2;
            let center_y = (e.touches[0].clientY + e.touches[1].clientY) / 2;

            let w = window.innerWidth;
            let h = window.innerHeight;

            center_x -= w/2;
            center_y -= h/2;
            center_y *= -1;

            if (w >= h) {
                center_x /= h;
                center_y /= h;
            } else {
                center_x /= w;
                center_y /= w;
            }

            center_x *= 2;
            center_y *= 2;

            canvas.add_to_r(ddist, center_x, center_y);

            prev_dist = cur_dist;
        });

        webgl_canvas.addEventListener("touchend", function(e) {
            prev_dist = 0.0;
            touch_zooming = false;
        });
    }

    console.log("Loading complete");
}

run();