use windows::{
    core::*,
    Win32::{System::{Com::*, Performance::QueryPerformanceFrequency}, Foundation::HWND, Graphics::{Direct2D::{ID2D1Factory1, ID2D1StrokeStyle, ID2D1DeviceContext, ID2D1SolidColorBrush, ID2D1Bitmap1, D2D1_FACTORY_OPTIONS, D2D1_DEBUG_LEVEL_INFORMATION, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1CreateFactory, D2D1_STROKE_STYLE_PROPERTIES}, Dxgi::{IDXGIFactory2, IDXGISwapChain1, CreateDXGIFactory1}}}
};

pub mod sample;

pub struct Window {
    handler: HWND,
    factory: ID2D1Factory1,
    dxfactory: IDXGIFactory2,
    style: ID2D1StrokeStyle,
    target: Option<ID2D1DeviceContext>,
    swapchain: Option<IDXGISwapChain1>,
    brush: Option<ID2D1SolidColorBrush>,
    img: Option<ID2D1Bitmap1>,
    dpi: f32,
    visible: bool,
    pending: u32,
}

impl Window {
    pub fn new() -> Result<Self> {
        let factory = create_factory()?;
        let dxfactory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
        let style = create_style(&factory)?;
        let mut dpi = 0.0;
        let mut dpiy = 0.0;
        unsafe { factory.GetDesktopDpi(&mut dpi, &mut dpiy) };
        let mut frequency = 0;
        unsafe { QueryPerformanceFrequency(&mut frequency)? };

        return Ok(Self {
            handler: HWND(0),
            factory,
            dxfactory,
            style,
            target: None,
            swapchain: None,
            brush: None,
            img: None,
            dpi,
            visible: false,
            pending: 0,
        })
;    }
}


fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES::default();

    unsafe { factory.CreateStrokeStyle(&props, None) }
}