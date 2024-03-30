
use winapi::um::d3d12::*;

pub struct PlatformRenderer {
    swap_chain: Box<IDXGISwapChain3>,
    device: *mut ID3D12Device,
    device_context: *mut ID3D12DeviceContext,
    render_target: *mut ID3D12RenderTargetView,
}