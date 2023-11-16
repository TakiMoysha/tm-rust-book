@fragment
fn main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
  var glsl_position:vec2<f32>=vec2<f32>(position.x,Resolution.y-position.y);
  var uv:vec2<f32>=vec2<f32>((glsl_position-(0.7*Resolution.xy))/Resolution.y);
  var x:f32=uv.x*3.0;
  var y:f32=uv.y*4.0;
  var m:f32= 1.0-(y*9.0)*(12.0+sin(-Time/4.0)*0.6)*0.7+sin(y*3.0+x*2.0-Time*9.0)*sin(y*1.7-x*6.0-Time+sin(x*3.0-y*0.3-Time*2.0)+cos(Time/6.0-y*6.0+x*6.0)/3.0)*69.0;
  return vec4<f32>(m*0.016,m*0.008,m*0.001,1.0);
}
