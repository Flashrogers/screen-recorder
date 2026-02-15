use windows::{
    core::ComInterface,
    Win32::{
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::*,
            Dxgi::Common::*,
        },
        System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    },
};

pub struct DxgiCapture {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    duplication: IDXGIOutputDuplication,
    width: u32,
    height: u32,
}

impl DxgiCapture {
    pub fn new() -> windows::core::Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;

            let mut device = None;
            let mut context = None;

            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                None,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )?;

            let device = device.unwrap();
            let context = context.unwrap();

            let dxgi_device: IDXGIDevice = device.cast()?;
            let adapter: IDXGIAdapter = dxgi_device.GetAdapter()?;
            let output: IDXGIOutput = adapter.EnumOutputs(0)?;
            let output1: IDXGIOutput1 = output.cast()?;

            let mut desc = DXGI_OUTPUT_DESC::default();
            output.GetDesc(&mut desc)?;

            let width =
                (desc.DesktopCoordinates.right - desc.DesktopCoordinates.left) as u32;

            let height =
                (desc.DesktopCoordinates.bottom - desc.DesktopCoordinates.top) as u32;

            let duplication = output1.DuplicateOutput(&device)?;

            Ok(Self {
                device,
                context,
                duplication,
                width,
                height,
            })
        }
    }

    pub fn capture_frame(&mut self) -> windows::core::Result<Vec<u8>> {
        unsafe {
            let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut resource: Option<IDXGIResource> = None;

            self.duplication
                .AcquireNextFrame(16, &mut frame_info, &mut resource)?;

            let resource = resource.unwrap();
            let texture: ID3D11Texture2D = resource.cast()?;

            let mut desc = D3D11_TEXTURE2D_DESC::default();
            texture.GetDesc(&mut desc);

            desc.Usage = D3D11_USAGE_STAGING;
            desc.BindFlags = 0;
            desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
            desc.MiscFlags = 0;

            let mut staging = None;
            self.device
                .CreateTexture2D(&desc, None, Some(&mut staging))?;

            let staging = staging.unwrap();

            self.context.CopyResource(&staging, &texture);

            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            self.context.Map(
                &staging,
                0,
                D3D11_MAP_READ,
                0,
                Some(&mut mapped),
            )?;

            let row_pitch = mapped.RowPitch as usize;
            let height = self.height as usize;
            let width = self.width as usize;

            let mut buffer = vec![0u8; width * height * 4];

            for y in 0..height {
                let src = std::slice::from_raw_parts(
                    (mapped.pData as *const u8).add(y * row_pitch),
                    width * 4,
                );

                let dst = &mut buffer[y * width * 4..(y + 1) * width * 4];
                dst.copy_from_slice(src);
            }

            self.context.Unmap(&staging, 0);
            self.duplication.ReleaseFrame()?;

            Ok(buffer)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
