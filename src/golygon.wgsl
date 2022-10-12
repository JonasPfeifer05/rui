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

    for (var i = 0; i < len; i++) {
                var x1 = lines[i].x1;
                var y1 = lines[i].y1;
                var x2 = lines[i].x2;
                var y2 = lines[i].y2;

                var x3 = v1x1;
                var y3 = v1y1;
                var x4 = x3 + 1.0;
                var y4 = y3 + 0.0;

                var den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                if (den == 0.0) {
                    continue;
                }

                var t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
                if (!(t > 0.0 && t < 1.0)) {
                    continue;
                }

                var u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

                if (u > 0.0) {
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