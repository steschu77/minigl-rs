use windows::core::*;
use windows::Win32::{
    Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::*,
};

pub fn loword(dword: u32) -> i32 {
    (dword & 0xffff) as i16 as i32
}

pub fn hiword(dword: u32) -> i32 {
    ((dword >> 16) & 0xffff) as i16 as i32
}

pub trait WindowProcTrait {
    type Params;
    fn create(hwnd: HWND, params: &Self::Params) -> Self;

    fn on_destroy(&mut self) -> LRESULT;
    fn on_size(&mut self, cx: i32, cy: i32) -> LRESULT;
    fn on_paint(&mut self, hdc: HDC, ps: &PAINTSTRUCT);
}

pub struct WindowProc<T> {
    hwnd: HWND,
    data: Box<T>,
}

impl<T: WindowProcTrait> WindowProc<T> {
    pub fn create(
        title: &str,
        class_name: &str,
        style: WINDOW_STYLE,
        params: &T::Params,
    ) -> Result<HWND> {
        let title = title.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
        let class_name = class_name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();

        let h_instance = unsafe { GetModuleHandleW(None).unwrap() };

        let wc = WNDCLASSW {
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
            hbrBackground: unsafe { HBRUSH(GetStockObject(NULL_BRUSH).0) },
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            style: CS_OWNDC,
            lpfnWndProc: Some(Self::wndproc),
            ..Default::default()
        };

        unsafe { RegisterClassW(&wc) };

        let params = Box::new(params);

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(title.as_ptr()),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                None,
                None,
                h_instance,
                Some(Box::into_raw(params) as *const core::ffi::c_void),
            )?
        };

        Ok(hwnd)
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_NCCREATE {
            let cs = lparam.0 as *const CREATESTRUCTW;
            let params = (*cs).lpCreateParams as *mut T::Params;
            let create_data = Box::from_raw(params);

            let data = Box::new(T::create(hwnd, &create_data));
            let window = Box::new(WindowProc { hwnd, data });
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(window) as isize);
        }

        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WindowProc<T>;
        if ptr.is_null() {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        } else {
            let result = ptr.as_mut().unwrap().handle_msg(msg, wparam, lparam);

            if msg == WM_NCDESTROY {
                // remove the object from the window and delete it
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);

                let window = Box::from_raw(ptr);
                drop(window); // make it obivous that the window is being deleted
            }
            result
        }
    }

    fn handle_msg(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_DESTROY => self.data.on_destroy(),
            WM_SIZE => self
                .data
                .on_size(loword(lparam.0 as u32), hiword(lparam.0 as u32)),
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = unsafe { BeginPaint(self.hwnd, &mut ps) };
                self.data.on_paint(hdc, &ps);
                let _ = unsafe { EndPaint(self.hwnd, &ps) };
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) },
        }
    }
}

pub fn run_message_loop() {
    let mut msg = MSG::default();
    unsafe {
        while GetMessageA(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            let _ = DispatchMessageA(&msg);
        }
    }
}
