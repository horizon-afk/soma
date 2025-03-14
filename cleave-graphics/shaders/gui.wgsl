struct VertexInput {
    @location(0) position: vec2<f32>,     // NDC coords (-1 to 1)
    @location(1) tex_coords: vec2<f32>,   // UV coords (0 to 1)
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct Uniforms {
    screen_size: vec2<f32>,
    drag_start: vec2<f32>,    // Screen coords (0 to width/height)
    drag_end: vec2<f32>,
    selection_start: vec2<f32>,
    selection_end: vec2<f32>,
    time: f32,
    is_dragging: u32,
};

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;
@group(1) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Pass through texture coordinates unchanged
    out.tex_coords = model.tex_coords;
    // Pass through position unchanged
    out.clip_position = vec4<f32>(model.position.xy, 0.0, 1.0);
    return out;
}

// Add this helper function for generating rainbow colors
fn rainbow(t: f32) -> vec4<f32> {
    let r = sin(t) * 0.5 + 0.5;
    let g = sin(t + 2.094) * 0.5 + 0.5; // 2.094 = 2π/3
    let b = sin(t + 4.189) * 0.5 + 0.5; // 4.189 = 4π/3
    return vec4<f32>(r, g, b, 0.8);
}


fn is_in_selection(coord: vec2<f32>) -> bool {
    let min_pos = min(uniforms.selection_start, uniforms.selection_end);
    let max_pos = max(uniforms.selection_start, uniforms.selection_end);
    return coord.x >= min_pos.x && coord.x <= max_pos.x && 
           coord.y >= min_pos.y && coord.y <= max_pos.y;
}

fn is_in_drag(coord: vec2<f32>) -> bool {
    let min_pos = min(uniforms.drag_start, uniforms.drag_end);
    let max_pos = max(uniforms.drag_start, uniforms.drag_end);
    return coord.x >= min_pos.x && coord.x <= max_pos.x && 
           coord.y >= min_pos.y && coord.y <= max_pos.y;
}

fn is_on_border(coord: vec2<f32>, region_start: vec2<f32>, region_end: vec2<f32>, thickness: f32) -> bool {
  let min_pos = min(region_start, region_end);
  let max_pos = max(region_start, region_end);
  
  // Check if near any of the four borders with dashed pattern
  let border_x = abs(coord.x - min_pos.x) < thickness || abs(coord.x - max_pos.x) < thickness;
  let border_y = abs(coord.y - min_pos.y) < thickness || abs(coord.y - max_pos.y) < thickness;
  
  if border_x || border_y {
    // Create dashed effect
    let dash_length = 10.0;
    let animation_speed = 20.0; // Change this variable to adjust the speed of the animation
    var pos: f32;
    if border_x {
      pos = coord.y + uniforms.time * animation_speed;
    } else {
      pos = coord.x + uniforms.time * animation_speed;
    }
    let dash_pattern = floor(pos / dash_length) % 2.0;
    return dash_pattern < 1.0;
  }
  
  return false;
}

fn get_stripe_pattern(coord: vec2<f32>) -> bool {
  let stripe_width = 10.0;  // Width of each stripe
  let stripe_spacing = 25.0; // Space between each stripe
  let animation_speed = 20.0; // Change this variable to adjust the speed of the animation
  let pos = (coord.x + coord.y + uniforms.time * animation_speed) / (stripe_width + stripe_spacing);
  return fract(pos) < (stripe_width / (stripe_width + stripe_spacing));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let coord = in.tex_coords * uniforms.screen_size;
    let tex = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    var color = tex;
    let border_thickness = 2.0;
    
    if (uniforms.is_dragging == 1u || uniforms.is_dragging == 3u) && is_in_drag(coord) {
        if is_on_border(coord, uniforms.drag_start, uniforms.drag_end, border_thickness) {
            color = vec4<f32>(0.0, 0.5, 1.0, 1.0);  // Blue border
        }
        //  else if get_stripe_pattern(coord) {
        //     color = mix(color, vec4<f32>(0.0, 0.5, 1.0, 0.3), 0.3);  // Semi-transparent blue stripes
        // }
    }
    
    if (uniforms.is_dragging == 2u || uniforms.is_dragging == 3u) && is_in_selection(coord) {
        if is_on_border(coord, uniforms.selection_start, uniforms.selection_end, border_thickness) {
            color = mix(color, vec4<f32>(0.0, 1.0, 0.0, 1.0), 0.5);  // Green border
        } else if get_stripe_pattern(coord) {
            color = mix(color, vec4<f32>(0.0, 0.5, 1.0, 0.3), 0.1);  // Semi-transparent blue stripes
        }
    }
    
    return color;
}