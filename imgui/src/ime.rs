use std::panic::catch_unwind;
use std::process;

pub trait ImeDataBackend: 'static {
    fn set_ime_data(
        &mut self,
        ctx: *mut sys::ImGuiContext,
        viewport: *mut sys::ImGuiViewport,
        data: *mut sys::ImGuiPlatformImeData,
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
        _: *mut sys::ImGuiViewport,
        _: *mut sys::ImGuiPlatformImeData,
    ) {
    }
}

pub(crate) unsafe extern "C" fn set_ime_data(
    imgui_ctx: *mut sys::ImGuiContext,
    viewport: *mut sys::ImGuiViewport,
    data: *mut sys::ImGuiPlatformImeData,
) {
    let result = catch_unwind(|| {
        let user_data = unsafe { (*sys::igGetPlatformIO()).Platform_ImeUserData };

        let ctx = &mut *(user_data as *mut ImeDataContext);
        ctx.backend.set_ime_data(imgui_ctx, viewport, data);
    });
    result.unwrap_or_else(|_| {
        eprintln!("IME data setter panicked");
        process::abort();
    });
}
