use crate::buffers::{BufferElement, BufferLayout, ShaderDataType};
use crate::shaders::{Shader, ShaderUniformSpec};
use crate::Renderer;

pub fn quad_shader_layout() -> BufferLayout {
    return BufferLayout::new(vec![BufferElement::new("POSITION", ShaderDataType::Float3)]);
}

pub fn quad_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer,
        quad_shader_layout(),
        QUAD_VERTEX_SHADER,
        QUAD_FRAGMENT_SHADER,
        ShaderUniformSpec { uniforms: vec![] },
        ShaderUniformSpec { uniforms: vec![] },
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const QUAD_VERTEX_SHADER: &str = "
using namespace metal;

struct VertexUniforms {
    float4x4 mat;
};

struct VertexIn {
    float2 xy   [[attribute(0)]];
};

struct ViewProjection {
    float4x4 mat;
};

struct ModelTransform {
    float4x4 mat;
};

vertex float4 vertex_main(constant VertexIn *in [[buffer(0)]],
                                 constant ViewProjection &viewProjection [[buffer(1)]],
                                 constant ModelTransform &modelTransform [[buffer(2)]],
                                 uint vid [[vertex_id]])
{
    return (viewProjection.mat * modelTransform.mat) * float4(in[vid].xy, 0.0, 1.0);
}
";

#[cfg(target_os = "windows")]
pub const QUAD_VERTEX_SHADER: &str = "
struct VSOut {
    float4 position : SV_POSITION;
};
cbuffer CBuf1
{
    matrix projection;
};

cbuffer CBuf2
{
    matrix transformation;
};

VSOut VSMain(float3 pos : Position)
{
    VSOut vso;
    float4x4 worldViewProj = mul(projection, transformation);
    vso.position = mul(worldViewProj, float4(pos.x, pos.y, pos.z, 1.0f));
    return vso;
}
";

#[cfg(target_os = "macos")]
pub const QUAD_FRAGMENT_SHADER: &str = "
using namespace metal;

fragment float4 fragment_main( constant float4 &color [[buffer(0)]])
{
    return float4(color[0], color[1], color[2], color[3]);
}
";

#[cfg(target_os = "windows")]
pub const QUAD_FRAGMENT_SHADER: &str = "
cbuffer CBuf
{
    float4 color;
};

float4 PSMain() : SV_Target
{
    return float4(color[0], color[1], color[2], color[3]);
}
";
