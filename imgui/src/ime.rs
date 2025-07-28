use std::panic::catch_unwind;
use std::process;

pub trait ImeDataBackend: 'static {
    fn set_ime_data(
        &mut self,
        ctx: *mut sys::ImGuiContext,
        viewport: &mut crate::Viewport,
        data: PlatformImeData,
    );
}

pub(crate) struct ImeDataContext {
    backend: Box<dyn ImeDataBackend>,
}

impl ImeDataContext {
    pub(crate) fn new<T: ImeDataBackend>(backend: T) -> ImeDataContext {
        ImeDataContext {
            backend: Box::new(backend) as Box<dyn ImeDataBackend>,
        }
    }

    pub(crate) fn dummy() -> ImeDataContext {
        ImeDataContext {
            backend: Box::new(DummyImeDataContext),
        }
    }
}

pub(crate) struct DummyImeDataContext;
impl ImeDataBackend for DummyImeDataContext {
    fn set_ime_data(
        &mut self,
        _: *mut sys::ImGuiContext,
        _: &mut crate::Viewport,
        _: PlatformImeData,
    ) {
    }
}

pub(crate) unsafe extern "C" fn set_ime_data(
    imgui_ctx: *mut sys::ImGuiContext,
    viewport: *mut sys::ImGuiViewport,
    data: *mut sys::ImGuiPlatformImeData,
) {
    let result = catch_unwind(|| {
        let data = unsafe { *(data as *mut crate::PlatformImeData).as_ref().unwrap() };

        let user_data = unsafe { (*sys::igGetPlatformIO_Nil()).Platform_ImeUserData };
        let ctx = &mut *(user_data as *mut ImeDataContext);

        ctx.backend
            .set_ime_data(imgui_ctx, &mut *(viewport as *mut crate::Viewport), data);
    });
    result.unwrap_or_else(|_| {
        eprintln!("IME data setter panicked");
        process::abort();
    });
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PlatformImeData {
    pub want_visible: bool,
    pub input_pos: [f32; 2],
    pub input_line_height: f32,
}
