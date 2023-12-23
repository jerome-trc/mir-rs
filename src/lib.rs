//! # mir-rs
//!
//! A safe, ergonomic Rust wrapper over [Vladimir Makarov's MIR toolchain].
//!
//! Neither this nor that project are related to Rust's [Mid-level IR].
//!
//! [Vladimir Makarov's MIR toolchain]: https://github.com/vnmakarov/mir
//! [Mid-level IR]: https://rustc-dev-guide.rust-lang.org/mir/index.html

pub extern crate sys;

use std::{ffi::c_int, ptr::NonNull};

/// Fully re-entrant state implicit to any usage of the MIR toolchain.
#[derive(Debug)]
pub struct Context {
	inner: NonNull<sys::MIR_context>,
	c2mir: bool,
}

impl Context {
	#[must_use]
	pub fn new(c2mir: bool) -> Self {
		unsafe {
			let inner = NonNull::new(sys::_MIR_init()).unwrap();

			if c2mir {
				sys::c2mir_init(inner.as_ptr());
			}

			Self { inner, c2mir }
		}
	}

	/// Note that MIR forces the following:
	/// - if `count` is zero, it will be silently set to 1.
	/// - MIR accepts a C `int`, so this will panic if `count` is greater
	/// than or equal to `c_int::MAX`.
	#[must_use]
	pub fn generators(&mut self, count: u32) -> Generators {
		unsafe {
			assert!(count < (c_int::MAX as u32));
			sys::MIR_gen_init(self.inner.as_ptr(), count as c_int);
			Generators { ctx: self, count }
		}
	}

	#[must_use]
	pub fn raw(&self) -> NonNull<sys::MIR_context> {
		self.inner
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			if self.c2mir {
				sys::c2mir_finish(self.inner.as_ptr());
			}

			sys::MIR_finish(self.inner.as_ptr());
		}
	}
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

#[derive(Debug)]
pub struct Generators<'ctx> {
	ctx: &'ctx mut Context,
	count: u32,
}

impl Generators<'_> {
	#[must_use]
	pub fn context(&self) -> &Context {
		self.ctx
	}

	#[must_use]
	pub fn count(&self) -> u32 {
		self.count
	}

	/// Panics if `generator` is out of the range of the initialized generators.
	pub fn set_optimization(&mut self, generator: u32, level: Optimization) {
		assert!(self.count > generator);

		unsafe {
			sys::MIR_gen_set_optimize_level(
				self.ctx.inner.as_ptr(),
				generator as i32,
				level as u32,
			);
		}
	}
}

impl Drop for Generators<'_> {
	fn drop(&mut self) {
		unsafe {
			sys::MIR_gen_finish(self.ctx.inner.as_ptr());
		}
	}
}

unsafe impl Send for Generators<'_> {}
unsafe impl Sync for Generators<'_> {}

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
		let _ = Context::new(true);
	}
}
