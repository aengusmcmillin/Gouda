use crate::rendering::{shader::Shader, buffers::{BufferLayout, BufferElement, ShaderDataType}, Renderer};


pub fn quad_shader(renderer: &Renderer) -> Shader {
    let buffer_layout = BufferLayout::new(
        vec![
            BufferElement::new("xy".to_string(), ShaderDataType::Float4),
        ]
    );
    let shader = Shader::new(
        renderer, 
        buffer_layout, 
        QUAD_VERTEX_SHADER,
        QUAD_FRAGMENT_SHADER,
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
    float4 xy   [[attribute(0)]];
};

vertex float4 vertex_main(constant VertexIn *in [[buffer(0)]],
                                 constant VertexUniforms &transformation [[buffer(1)]],
                                 constant VertexUniforms &projection [[buffer(2)]],
                                 uint vid [[vertex_id]])
{
    return float4(in[vid].xy) * transformation.mat * projection.mat;
}
";

#[cfg(target_os = "windows")]
pub const QUAD_VERTEX_SHADER: &str = "
cbuffer CBuf
{
    matrix transformation;
};

cbuffer CBuf2
{
    matrix projection;
};

float4 main(float4 pos : POSITION) : SV_POSITION
{
    return mul(mul(float4(pos.x, pos.y, 0.0f, 1.0f), transformation), projection);
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

float4 main() : SV_TARGET
{
    return float4(color[0], color[1], color[2], color[3]);
}
";