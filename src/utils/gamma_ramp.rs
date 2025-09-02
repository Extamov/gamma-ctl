use itertools::izip;

pub type GammaRamp = [[u16; 256]; 3];

pub struct GammaRampIter {
	range: f64,
	offset: f64,
	inv_gamma: f64,
	index: u16,
}
impl GammaRampIter {
	pub fn new(brightness: f64, contrast: f64, gamma: f64) -> Self {
		let adjusted_brightness = (brightness / 50.0) - 1.0;
		let adjusted_contrast = (contrast / 50.0) - 1.0;
		let inv_gamma = 1.0 / gamma;
		let mut offset = if adjusted_contrast > 0.0 {
			adjusted_contrast * -25.4
		} else {
			adjusted_contrast * -32.0
		};
		let range = 255.0 + offset * 2.0;
		offset += adjusted_brightness * (range / 5.0);
		Self {
			range,
			offset,
			inv_gamma,
			index: 0,
		}
	}
}
impl Iterator for GammaRampIter {
	type Item = u16;
	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= 256 {
			return None;
		}
		let factor = ((self.index as f64 + self.offset) / self.range).powf(self.inv_gamma);
		self.index += 1;
		Some((factor.max(0.0).min(1.0) * 65535.0) as u16)
	}
}

pub fn generate_gamma_ramp(brightness: f64, contrast: f64, gamma: f64) -> GammaRamp {
	let mut gamma_ramp = [[0u16; 256]; 3];
	let [reds, greens, blues] = &mut gamma_ramp;
	let gamma_ramp_iter = GammaRampIter::new(brightness, contrast, gamma);
	for (output, red, green, blue) in izip!(gamma_ramp_iter, reds, greens, blues) {
		*red = output;
		*green = output;
		*blue = output;
	}
	gamma_ramp
}
