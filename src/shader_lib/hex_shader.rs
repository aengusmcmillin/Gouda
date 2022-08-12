use crate::rendering::{shader::Shader, buffers::{BufferLayout, BufferElement, ShaderDataType}, Renderer};


pub fn hex_shader(renderer: &Renderer) -> Shader {
    let buffer_layout = BufferLayout::new(
        vec![
            BufferElement::new("position".to_string(), ShaderDataType::Float2),
        ]
    );
    let shader = Shader::new(
        renderer, 
        buffer_layout, 
        HEX_VERTEX_SHADER,
        HEX_FRAGMENT_SHADER,
    );
    return shader;
}

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
    constant ViewProjection& viewProjection [[buffer(1)]],
    constant ModelTransform& modelTransform [[buffer(2)]]) {
    return viewProjection.vp * modelTransform.transform * float4(vIn.position, 0, 1);
}
";

pub const HEX_FRAGMENT_SHADER: &str = "
using namespace metal;

fragment float4 fragment_main( constant float4 &color [[buffer(0)]])
{
    return float4(color[0], color[1], color[2], color[3]);
}
";