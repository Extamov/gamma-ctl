use super::{GammaRamp, StrResult};
use std::ptr::{null, null_mut};
use windows_sys::Win32::{
	Graphics::Gdi::{CreateDCA, DeleteDC, HDC},
	UI::ColorSystem::SetDeviceGammaRamp,
};

pub struct Monitor(HDC);
impl Monitor {
	pub fn new(index: u8) -> StrResult<Self> {
		if index == 0 || index >= 10 {
			return Err("Monitor index must be between 1 and 9 (inclusive)");
		}

		// The null terminated string r"\\.\DISPLAY{index}"
		let monitor_name = {
			let mut name = [0u8; 13];
			name[..11].copy_from_slice(r"\\.\DISPLAY".as_bytes());
			name[11] = index + 48;
			name
		};

		let hdc = unsafe { CreateDCA(null(), monitor_name.as_ptr(), null(), null()) };
		if hdc == null_mut() {
			return Err("Failed to find monitor with specified index");
		}
		Ok(Self(hdc))
	}
	pub fn set_gamma_ramp(&self, gamma_ramp: &GammaRamp) -> StrResult<()> {
		if unsafe { SetDeviceGammaRamp(self.0, gamma_ramp.as_ptr() as *const _) } != 1 {
			return Err("Failed to set gamma settings");
		}
		Ok(())
	}
}
impl Drop for Monitor {
	fn drop(&mut self) {
		if unsafe { DeleteDC(self.0) } != 1 {
			panic!("Failed to delete monitor handle");
		}
	}
}
