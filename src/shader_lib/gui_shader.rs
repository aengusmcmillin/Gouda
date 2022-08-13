use crate::rendering::{shader::Shader, buffers::{BufferLayout, BufferElement, ShaderDataType}, Renderer};


pub fn gui_shader(renderer: &Renderer) -> Shader {
    let buffer_layout = BufferLayout::new(
        vec![
            BufferElement::new("position".to_string(), ShaderDataType::Float4),
            BufferElement::new("texCoord".to_string(), ShaderDataType::Float2)
        ]
    );
    let shader = Shader::new(
        renderer, 
        buffer_layout, 
        GUI_VERTEX_SHADER,
        GUI_FRAGMENT_SHADER,
    );
    return shader;
}


#[cfg(target_os = "macos")]
pub const GUI_VERTEX_SHADER: &str = "
using namespace metal;

struct VertexUniforms {
    float4x4 mat;
};

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

struct VertexIn {
    float4 position   [[attribute(0)]];
    float2 texCoord   [[attribute(1)]];
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]],
                                 constant VertexUniforms &transformation [[buffer(1)]],
                                 constant VertexUniforms &projection [[buffer(2)]])
{
    VertexOut VertexOut;
    VertexOut.position = in.position * transformation.mat * projection.mat;
    VertexOut.texCoord = in.texCoord;
    return VertexOut;
}
";

#[cfg(target_os = "windows")]
pub const GUI_VERTEX_SHADER: &str = "
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

float4 main() : SV_TARGET
{
    return float4(color[0], color[1], color[2], color[3]);
}
";