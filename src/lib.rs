pub mod error;
pub mod subclass;
pub mod wrapper;

pub use winapi;

pub mod prelude {
    pub use crate::{error::SubclassError, subclass::Subclass, wrapper::SubclassWrapper};

    pub use winapi::{
        ctypes::c_void,
        shared::{
            basetsd::UINT_PTR,
            minwindef::{LPARAM, LRESULT, UINT, WPARAM},
            windef::HWND,
        },
        DEFINE_GUID,
    };

    #[cfg(feature = "device")]
    pub use winapi::{
        shared::{
            guiddef::*,
            hidclass::GUID_DEVINTERFACE_HID,
            usbiodef::{
                GUID_DEVINTERFACE_USB_DEVICE, GUID_DEVINTERFACE_USB_HOST_CONTROLLER,
                GUID_DEVINTERFACE_USB_HUB,
            },
        },
        um::{
            portabledevice::{
                GUID_DEVINTERFACE_WPD, GUID_DEVINTERFACE_WPD_PRIVATE, GUID_DEVINTERFACE_WPD_SERVICE,
            },
            winioctl::{
                GUID_DEVINTERFACE_CDCHANGER, GUID_DEVINTERFACE_CDROM, GUID_DEVINTERFACE_COMPORT,
                GUID_DEVINTERFACE_DISK, GUID_DEVINTERFACE_FLOPPY, GUID_DEVINTERFACE_HIDDEN_VOLUME,
                GUID_DEVINTERFACE_MEDIUMCHANGER, GUID_DEVINTERFACE_PARTITION,
                GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE, GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR,
                GUID_DEVINTERFACE_SERVICE_VOLUME, GUID_DEVINTERFACE_SES,
                GUID_DEVINTERFACE_STORAGEPORT, GUID_DEVINTERFACE_TAPE,
                GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB, GUID_DEVINTERFACE_VMLUN,
                GUID_DEVINTERFACE_VOLUME, GUID_DEVINTERFACE_WRITEONCEDISK,
            },
            winsmcrd::GUID_DEVINTERFACE_SMARTCARD_READER,
            wlanapi::{GUID_DEVINTERFACE_ASP_INFRA_DEVICE, GUID_DEVINTERFACE_WIFIDIRECT_DEVICE},
        },
    };
}

#[cfg(not(target_os = "windows"))]
compile_error!("Target OS unsupported");
