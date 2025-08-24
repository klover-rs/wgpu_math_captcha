struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) in_pos: vec2<f32>,
    @location(1) in_color: vec3<f32>
) -> VertexOut {
    var out: VertexOut;
    out.pos = vec4<f32>(in_pos, 0.0, 1.0);
    out.color = in_color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0); // just output vertex color
}