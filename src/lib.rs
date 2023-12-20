//! # mir-rs
//!
//! A safe, ergonomic Rust wrapper over [Vladimir Makarov's MIR toolchain].
//!
//! Neither this nor that project are related to Rust's [Mid-level IR].
//!
//! [Vladimir Makarov's MIR toolchain]: https://github.com/vnmakarov/mir
//! [Mid-level IR]: https://rustc-dev-guide.rust-lang.org/mir/index.html

pub extern crate sys;

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

	/// Panics if `generator` is out of the range of the initialized generators.
	pub fn set_optimization(&self, generator: u32, level: Optimization) {
		assert!(self.gen_count > generator);

		unsafe {
			sys::MIR_gen_set_optimize_level(self.inner, generator as i32, level as u32);
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

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
	I8 = sys::MIR_type_t_MIR_T_I8,
	U8 = sys::MIR_type_t_MIR_T_U8,
	I16 = sys::MIR_type_t_MIR_T_I16,
	U16 = sys::MIR_type_t_MIR_T_U16,
	I32 = sys::MIR_type_t_MIR_T_I32,
	U32 = sys::MIR_type_t_MIR_T_U32,
	I64 = sys::MIR_type_t_MIR_T_I64,
	U64 = sys::MIR_type_t_MIR_T_U64,

	Float = sys::MIR_type_t_MIR_T_F,
	Double = sys::MIR_type_t_MIR_T_D,
	/// Machine-dependent; can be an IEEE double, x864 80-bit floating-point or
	/// IEEE quad-precision floating-point values.
	LongDouble = sys::MIR_type_t_MIR_T_LD,

	Pointer = sys::MIR_type_t_MIR_T_P,

	Block = sys::MIR_type_t_MIR_T_BLK,
	/// Return block data, only usable for function arguments.
	RBlock = sys::MIR_type_t_MIR_T_RBLK,
}

impl DataType {
	/// Note that this returns `true` for [`DataType::Pointer`].
	#[must_use]
	pub fn is_integer(self) -> bool {
		matches!(
			self,
			Self::I8
				| Self::U8 | Self::I16
				| Self::U16 | Self::I32
				| Self::U32 | Self::I64
				| Self::U64 | Self::Pointer
		)
	}

	/// Checks if this is one of the following:
	/// - [`DataType::Float`]
	/// - [`DataType::Double`]
	/// - [`DataType::LongDouble`]
	#[must_use]
	pub fn is_float(self) -> bool {
		matches!(self, Self::Float | Self::Double | Self::LongDouble)
	}
}

/// How much should each generator trade compile performance for runtime performance?
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Optimization {
	/// Only perform register allocation and machine code generation.
	None,
	/// More compact and faster code than [`Optimization::None`],
	/// at practically the same compilation speed.
	L1,
	/// Additional common sub-expression elimination and sparse conditional
	/// constant propagation. Note that this is the default.
	#[default]
	L2,
	/// Additional register renaming and loop-invariant code motion.
	/// [`Optimization::L2`] is about 50% faster than this.
	L3,
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn context_smoke() {
		let _ = Context::new(1);
	}
}
