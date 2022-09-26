use crate::rendering::{shader::Shader, buffers2::{BufferLayout, BufferElement, ShaderDataType}, Renderer};

pub fn gui_shader_layout() -> BufferLayout {
    return BufferLayout::new(
        vec![
            BufferElement::new("position", ShaderDataType::Float2),
        ]
    )
}

pub fn gui_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer, 
        gui_shader_layout(), 
        GUI_VERTEX_SHADER,
        GUI_FRAGMENT_SHADER,
    );
    return shader;
}


#[cfg(target_os = "macos")]
pub const GUI_VERTEX_SHADER: &str = "
using namespace metal;

struct ModelTransform {
    float4x4 transform;
};

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

struct VertexIn {
    float2 position [[attribute(0)]];
};

vertex VertexOut vertex_main(const VertexIn in [[stage_in]],
                                 constant ModelTransform &transform [[buffer(1)]])
{
    VertexOut VertexOut;
    VertexOut.position = transform.transform * float4(in.position, 0.0, 1.0);
    VertexOut.texCoord = float2((in.position.x) / 2.0, 1.0 - (in.position.y / 2.0));
    return VertexOut;
}
";

#[cfg(target_os = "windows")]
pub const GUI_VERTEX_SHADER: &str = "
struct VertexOut {
    float4 position : SV_POSITION;
    float2 texCoord : TEXCOORD;
};

cbuffer ModelTransform {
    matrix transform;
};

VertexOut VSMain(float2 position : Position)
{
    VertexOut VertexOut;
    VertexOut.position = mul(transform, float4(position.x, position.y, 0.0, 1.0));
    VertexOut.texCoord = float2((position.x) / 2.0, 1.0 - (position.y / 2.0));
    return VertexOut;
}
";

#[cfg(target_os = "macos")]
pub const GUI_FRAGMENT_SHADER: &str = "
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

float square(float val)
{
    return val * val;
}

float calc_rounded_corners(float2 coord, float radius, float width, float height)
{
    if (radius <= 0.0) {
        return 1.0;
    }
    float cornerSmooth = 0.002;

    float2 pixelPos = coord * float2(width, height);
    float2 minCorner = float2(radius, radius);
    float2 maxCorner = float2(width - radius, height - radius);

    float2 cornerPoint = clamp(pixelPos, minCorner, maxCorner);
    float lowerBound = square(radius - cornerSmooth);
    float upperBound = square(radius + cornerSmooth);
    return smoothstep(upperBound, lowerBound, distance_squared(pixelPos, cornerPoint));
}

fragment float4 fragment_main(VertexOut interpolated [[stage_in]], constant float4 &color [[buffer(0)]], constant float2 &dimensions [[buffer(1)]], constant float &radius [[buffer(2)]])
{
    float alpha = calc_rounded_corners(interpolated.texCoord, radius, dimensions.x, dimensions.y) * color[3];
    return float4(color[0], color[1], color[2], alpha);
}
";

#[cfg(target_os = "windows")]
pub const GUI_FRAGMENT_SHADER: &str = "
cbuffer CBuf
{
    float4 color;
};

cbuffer CBuf2
{
    float2 dimensions;
};

cbuffer CBuf3
{
    float radius;
}

struct VertexOut {
    float4 position : SV_POSITION;
    float2 texCoord : TEXCOORD;
};

float square(float val)
{
    return val * val;
}

float calc_rounded_corners(float2 coord, float radius, float width, float height)
{
    if (radius <= 0.0) {
        return 1.0;
    }
    float cornerSmooth = 0.002;

    float2 pixelPos = coord * float2(width, height);
    float2 minCorner = float2(radius, radius);
    float2 maxCorner = float2(width - radius, height - radius);

    float2 cornerPoint = clamp(pixelPos, minCorner, maxCorner);
    float lowerBound = square(radius - cornerSmooth);
    float upperBound = square(radius + cornerSmooth);
    return smoothstep(upperBound, lowerBound, square(distance(pixelPos, cornerPoint)));
}

float4 PSMain(VertexOut interpolated) : SV_TARGET
{
    float alpha = calc_rounded_corners(interpolated.texCoord, radius, dimensions.x, dimensions.y) * color[3];
    return float4(color[0], color[1], color[2], alpha);
}
";