// Vertex shader

struct Line {
    x1: f32,
    y1: f32,

    x2: f32,
    y2: f32,
};

struct Screen {
    width: f32,
    height: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@group(0) @binding(2)
var<uniform> screen: Screen;

@group(0) @binding(1)
var<uniform> len: i32;

@group(0) @binding(0)
var<uniform> lines: array<Line, 1000>;

fn is_inside_polygon(v1x1: f32, v1y1: f32) -> bool {
    var intersects = 0;

/*
    var xmin = 0.0;
    for (var i = 0; i < len; i++) {
        if (lines[i].x1 < xmin) {
            xmin = lines[i].x1;
        }
        if (lines[i].x2 < xmin) {
            xmin = lines[i].x2;
        }
    }
*/

    for (var i = 0; i < len; i++) {

                /*
                var v1x2 = xmin - 1.0;
                var v1y2 = v1y1;

                var v2x1 = lines[i].x1;
                var v2x2 = lines[i].x2;
                var v2y1 = lines[i].y1;
                var v2y2 = lines[i].y2;

                var a1 = v1y2 - v1y1;
                var b1 = v1x1 - v1x2;
                var c1 = (v1x2 * v1y1) - (v1x1 * v1y2);

                var d1 = (a1 * v2x1) + (b1 * v2y1) + c1;
                var d2 = (a1 * v2x2) + (b1 * v2y2) + c1;

                if (d1 > 0.0f && d2 > 0.0f) {continue;}
                if (d1 < 0.0f && d2 < 0.0f) {continue;}

                var a2 = v2y2 - v2y1;
                var b2 = v2x1 - v2x2;
                var c2 = (v2x2 * v2y1) - (v2x1 - v2y2);

                d1 = (a2 * v1x1) + (b2 * v1y1) + c2;
                d2 = (a2 * v1x2) + (b2 * v1y2) + c2;

                if (d1 > 0.0f && d2 > 0.0f) {continue;}
                if (d1 < 0.0f && d2 < 0.0f) {continue;}

                if ((a1 * b2) - (a2 * b1) == 0.0f) {continue;}

                intersects += 1;
                */

                var x1 = lines[i].x1;
                var y1 = lines[i].y1;
                var x2 = lines[i].x2;
                var y2 = lines[i].y2;

                var x3 = v1x1;
                var y3 = v1y1;
                var x4 = x3 + 10.0;
                var y4 = y3 + 0.0;

                var den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                if (den == 0.0) {
                    continue;
                }

                var t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
                var u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

                if (t > 0.0 && t < 1.0 && u > 0.0) {
                    intersects++;
                }
    }

    return (intersects & 1) == 1;
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;

    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec4<f32>(in.color, 0.0);
    if (is_inside_polygon((in.clip_position.x / screen.width)* 2.0 - 1.0, -((in.clip_position.y / screen.height)* 2.0 - 1.0))) {
        color.w = 1.0;
    }

    return color;
}