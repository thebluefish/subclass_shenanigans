use regex::Regex;
use widestring::U16CStr;
use win_subclass_shenanigans::prelude::*;
use win_subclass_shenanigans::winapi::um::{commctrl, dbt, winuser};
use winit::platform::windows::WindowExtWindows;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct GcnSubclass {
    regex: Regex,
}

impl GcnSubclass {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"^\\\\\?\\USB#VID_([0-9a-fA-F]+)&PID_([0-9a-fA-F]+)#").unwrap(),
        }
    }
}

impl Subclass for GcnSubclass {
    fn subclass_callback(
        &mut self,
        _: &mut bool,
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
        _: UINT_PTR,
    ) -> LRESULT {
        match msg {
            winuser::WM_DEVICECHANGE => match wparam {
                dbt::DBT_DEVICEARRIVAL | dbt::DBT_DEVICEREMOVECOMPLETE => {
                    let data = unsafe { &mut *(lparam as *mut dbt::DEV_BROADCAST_HDR) };
                    match data.dbch_devicetype {
                        dbt::DBT_DEVTYP_DEVICEINTERFACE => {
                            let data = unsafe {
                                &mut *(lparam as *mut dbt::DEV_BROADCAST_DEVICEINTERFACE_W)
                            };
                            let name = unsafe {
                                U16CStr::from_ptr_str(&data.dbcc_name as *const u16)
                                    .to_string_lossy()
                            };
                            let captures = self.regex.captures(&name).unwrap();

                            if let (Some(vid), Some(pid)) = (captures.get(1), captures.get(2)) {
                                println!(
                                    "device {}, vid: {} pid: {}",
                                    if wparam == dbt::DBT_DEVICEARRIVAL {
                                        "arrived"
                                    } else {
                                        "removed"
                                    },
                                    vid.as_str(),
                                    pid.as_str()
                                );
                            }
                        }
                        _ => {}
                    }

                    0
                }
                _ => unsafe { commctrl::DefSubclassProc(hwnd, msg, wparam, lparam) },
            },
            _ => unsafe { commctrl::DefSubclassProc(hwnd, msg, wparam, lparam) },
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut subclass = SubclassWrapper::new(window.hwnd() as HWND, 0, GcnSubclass::new());
    subclass.register_device_notification(Some(GUID_DEVINTERFACE_USB_DEVICE));

    if let Err((e, _)) = subclass.install() {
        println!("failed to install subclass, hotplug unsupported\n{:?}", e);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
