//! # mir-rs
//!
//! A safe, ergonomic Rust wrapper over [Vladimir Makarov's MIR toolchain].
//!
//! Neither this nor that project are related to Rust's [Mid-level IR].
//!
//! [Vladimir Makarov's MIR toolchain]: https://github.com/vnmakarov/mir
//! [Mid-level IR]: https://rustc-dev-guide.rust-lang.org/mir/index.html

/// Fully re-entrant state implicit to any usage of the MIR toolchain.
#[derive(Debug)]
pub struct Context {
	inner: sys::MIR_context_t,
	gen_count: u32,
}

impl Context {
	/// Note that MIR forces the following:
	/// - if the `parallel` feature flag is disabled,
	/// `generators` will always be set to 1.
	/// - `generators` is zero, it will be silently set to 1.
	#[must_use]
	pub fn new(generators: u32) -> Self {
		unsafe {
			let inner = sys::_MIR_init();
			sys::MIR_gen_init(inner, generators as i32);

			Self {
				inner,
				gen_count: generators,
			}
		}
	}

	/// How many code generators was this context initialized with?
	#[must_use]
	pub fn generator_count(&self) -> u32 {
		self.gen_count
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			sys::MIR_finish(self.inner);
		}
	}
}

unsafe impl Send for Context {}

#[cfg(feature = "parallel")]
unsafe impl Sync for Context {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn context_smoke() {
		let _ = Context::new(1);
	}
}
