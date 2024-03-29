use crate::buffers::{BufferElement, BufferLayout, ShaderDataType};
use crate::shaders::{Shader, ShaderUniformSpec};
use crate::Renderer;

pub fn hex_shader(renderer: &Renderer) -> Shader {
    let buffer_layout =
        BufferLayout::new(vec![BufferElement::new("position", ShaderDataType::Float2)]);
    let shader = Shader::new(
        renderer,
        buffer_layout,
        HEX_VERTEX_SHADER,
        HEX_FRAGMENT_SHADER,
        ShaderUniformSpec { uniforms: vec![] },
        ShaderUniformSpec { uniforms: vec![] },
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const HEX_VERTEX_SHADER: &str = "
using namespace metal;

struct ViewProjection {
    float4x4 vp;
};

struct ModelTransform {
    float4x4 transform;
};

struct VertexIn {
    float2 position [[ attribute(0) ]];
};

vertex float4 vertex_main(
    const VertexIn vIn [[ stage_in ]],
    constant ModelTransform& modelTransform [[buffer(1)]],
    constant ViewProjection& viewProjection [[buffer(2)]]) {
    return viewProjection.vp * modelTransform.transform * float4(vIn.position, 0, 1);
}
";

#[cfg(target_os = "windows")]
pub const HEX_VERTEX_SHADER: &str = "
cbuffer CBuf
{
    matrix transformation;
};

cbuffer CBuf2
{
    matrix projection;
};

float4 VSMain(float4 pos : POSITION) : SV_POSITION
{
    return mul(mul(float4(pos.x, pos.y, 0.0f, 1.0f), transformation), projection);
}
";

#[cfg(target_os = "macos")]
pub const HEX_FRAGMENT_SHADER: &str = "
using namespace metal;

fragment float4 fragment_main( constant float4 &color [[buffer(0)]])
{
    return float4(color[0], color[1], color[2], color[3]);
}
";

#[cfg(target_os = "windows")]
pub const HEX_FRAGMENT_SHADER: &str = "
cbuffer CBuf
{
    float4 color;
};

float4 PSMain() : SV_TARGET
{
    return float4(color[0], color[1], color[2], color[3]);
}
";
