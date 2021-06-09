use crate::prelude::*;
use winapi::um::commctrl;

pub trait Subclass {
    /// You must defer any call you want the window to receive
    ///
    /// Defers the call by default
    fn subclass_callback(
        &mut self,
        _remove: &mut bool,
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
        _id: UINT_PTR,
    ) -> LRESULT {
        unsafe { commctrl::DefSubclassProc(hwnd, msg, wparam, lparam) }
    }
}
