fn main() {
	divan::main();
}

/// Test the speed of the wrapped functions to have a reference point for
/// measuring the overhead added by the wrapper.
mod wrapperless {
	use std::ffi::CString;

	#[divan::bench]
	fn context_creation() {
		unsafe {
			let _ = divan::black_box(sys::_MIR_init());
		}
	}

	#[divan::bench]
	fn module_creation(bencher: divan::Bencher) {
		unsafe {
			let mctx = sys::_MIR_init();
			let name = CString::new("bench").unwrap();
			bencher.bench_local(|| {
				let ret = sys::MIR_new_module(mctx, name.as_ptr());
				sys::MIR_finish_module(mctx);
				divan::black_box(ret)
			});
		}
	}
}
