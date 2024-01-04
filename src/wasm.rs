use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Instant(Duration);

impl Eq for Instant {}

impl Instant {
	#[inline]
	pub fn now() -> Self {
		Instant(duration_from_f64(now()))
	}
	#[inline]
	pub fn elapsed(&self) -> Duration {
		Self::now().duration_since(*self)
	}
	#[inline]
	pub fn duration_since(&self, earlier: Instant) -> Duration {
		self.0 - earlier.0
	}
	#[inline]
	pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
		self.0 - earlier.0
	}
	#[inline]
	pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
		self.0.checked_add(duration).map(Instant)
	}
	#[inline]
	pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
		self.0.checked_sub(duration).map(Instant)
	}
}

fn duration_from_f64(millis: f64) -> Duration {
	Duration::from_millis(millis.trunc() as u64)
		+ Duration::from_nanos((millis.fract() * 1.0e6) as u64)
}

fn now() -> f64 {
	js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
		.expect("failed to get performance from global object")
		.unchecked_into::<web_sys::Performance>()
		.now()
}

impl std::ops::Add<Duration> for Instant {
	type Output = Instant;
	fn add(self, other: Duration) -> Instant {
		Instant(self.0.add(other))
	}
}

impl std::ops::Sub<Duration> for Instant {
	type Output = Instant;
	fn sub(self, other: Duration) -> Instant {
		Instant(self.0.sub(other))
	}
}
