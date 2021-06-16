use crate::prelude::*;
use is_main_thread::is_main_thread;
use winapi::{
    shared::{basetsd::DWORD_PTR, ntdef::NULL},
    um::{commctrl, dbt, winuser},
};

pub struct SubclassWrapper<T>
where
    T: Subclass,
{
    pub id: usize,
    pub hwnd: HWND,
    pub data: T,
    removed: bool,
    recurse_depth: u32,
    #[cfg(feature = "device")]
    pub notification_handles: Vec<*mut c_void>,
}

#[cfg(feature = "device")]
impl<T> Drop for SubclassWrapper<T>
where
    T: Subclass,
{
    fn drop(&mut self) {
        for handle in &self.notification_handles {
            unsafe {
                winuser::UnregisterDeviceNotification(*handle);
            }
        }
    }
}

impl<T> SubclassWrapper<T>
where
    T: Subclass,
{
    pub fn new(hwnd: HWND, id: usize, data: T) -> Self {
        Self {
            removed: false,
            recurse_depth: 0,
            id,
            hwnd,
            data,
            #[cfg(feature = "device")]
            notification_handles: vec![],
        }
    }

    /// Installs subclass to the window
    /// - Must be on the main thread
    /// - Must pass a valid HWND
    pub fn install(self) -> Result<(), (SubclassError, Self)> {
        match is_main_thread() {
            Some(true) | None => unsafe {
                let data = Box::into_raw(Box::new(self));
                let result = commctrl::SetWindowSubclass(
                    (*data).hwnd,
                    Some(Self::callback),
                    (*data).id,
                    data as DWORD_PTR,
                );

                if result == 0 {
                    Err((SubclassError::InstallFailed, *Box::from_raw(data)))
                } else {
                    Ok(())
                }
            },
            Some(false) => Err((SubclassError::NotMainThread, self)),
        }
    }

    extern "system" fn callback(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
        id: UINT_PTR,
        data: DWORD_PTR,
    ) -> LRESULT {
        let this = unsafe { &mut *(data as *mut Self) };

        this.recurse_depth += 1;

        let mut remove = false;

        let ret = this
            .data
            .subclass_callback(&mut remove, hwnd, msg, wparam, lparam, id);

        if msg == winuser::WM_NCDESTROY || remove {
            unsafe {
                commctrl::RemoveWindowSubclass(hwnd, Some(Self::callback), this.id);
            }
            this.removed = true;
        }

        this.recurse_depth -= 1;

        if let SubclassWrapper {
            removed: true,
            recurse_depth: 0,
            ..
        } = this
        {
            unsafe {
                Box::from_raw(this);
            }
        }

        ret
    }

    #[cfg(feature = "device")]
    pub fn register_device_notification(&mut self, guid_filter: Option<GUID>) -> &mut Self {
        let dbcc_classguid = if let Some(guid) = guid_filter {
            guid
        } else {
            IID_NULL
        };

        let flags = if let None = guid_filter {
            winuser::DEVICE_NOTIFY_ALL_INTERFACE_CLASSES
        } else {
            winuser::DEVICE_NOTIFY_WINDOW_HANDLE
        };

        let mut filter = dbt::DEV_BROADCAST_DEVICEINTERFACE_A {
            dbcc_size: std::mem::size_of::<dbt::DEV_BROADCAST_DEVICEINTERFACE_A>() as u32,
            dbcc_devicetype: dbt::DBT_DEVTYP_DEVICEINTERFACE,
            dbcc_reserved: 0,
            dbcc_classguid,
            dbcc_name: [0],
        };

        unsafe {
            let r = winuser::RegisterDeviceNotificationA(
                self.hwnd as *mut c_void,
                &mut filter as *mut _ as *mut c_void,
                flags,
            );

            if r == NULL {
                println!("failed to register device");
            } else {
                self.notification_handles.push(r);
            }
        }

        self
    }
}
