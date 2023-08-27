//! # mircomp-sys
//!
//! Auto-generated FFI bindings to [Vladimir Makarov's MIR](https://github.com/vnmakarov/mir) toolchain.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // `u128`

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
	use std::ffi::{c_char, c_int, c_void, CStr, CString};

	use super::*;

	#[test]
	fn smoke() {
		unsafe {
			assert_eq!(MIR_API_VERSION, _MIR_get_api_version());
			let mctx = _MIR_init();
			c2mir_init(mctx);
			MIR_gen_init(mctx, 1);

			let mut options = c2mir_options {
				message_file: stderr,
				debug_p: i32::from(false),
				verbose_p: i32::from(false),
				ignore_warnings_p: i32::from(false),
				no_prepro_p: i32::from(false),
				prepro_only_p: i32::from(false),
				syntax_only_p: i32::from(false),
				pedantic_p: i32::from(false),
				asm_p: i32::from(false),
				object_p: i32::from(false),
				module_num: 0,
				prepro_output_file: std::ptr::null_mut(),
				output_file_name: std::ptr::null_mut(),
				macro_commands_num: 0,
				include_dirs_num: 0,
				macro_commands: std::ptr::null_mut(),
				include_dirs: std::ptr::null_mut(),
			};

			#[derive(Debug)]
			struct Buffer {
				pos: usize,
				code: *const c_char,
			}

			unsafe extern "C" fn getc_func(userd: *mut c_void) -> c_int {
				let buf: *mut Buffer = userd.cast();
				let p = (*buf).pos;
				let mut c = *(*buf).code.add(p);

				if c == 0 {
					c = -1; // libc's EOF
				} else {
					(*buf).pos += 1;
				}

				c as c_int
			}

			unsafe extern "C" fn import_resolver(_: *const c_char) -> *mut c_void {
				std::ptr::null_mut()
			}

			const SOURCE: &str = r#"
void rsprint(const char*);
int _main(void) { rsprint("hello rust"); return 999; }
"#;

			let c_src = CString::new(SOURCE).unwrap();

			let mut srcbuf = Buffer {
				code: c_src.as_ptr(),
				pos: 0,
			};

			let src_name = CString::new("smoke").unwrap();

			let result = c2mir_compile(
				mctx,
				std::ptr::addr_of_mut!(options),
				Some(getc_func),
				std::ptr::addr_of_mut!(srcbuf).cast(),
				src_name.as_ptr(),
				std::ptr::null_mut(),
			);

			assert_ne!(result, 0);

			let all_modules = MIR_get_module_list(mctx);
			let to_link = (*all_modules).head;
			MIR_load_module(mctx, to_link);

			unsafe extern "C" fn rsprint(string: *const c_char) {
				let c_str = CStr::from_ptr(string);
				let rs_str = c_str.to_string_lossy();
				println!("{}", rs_str.as_ref());
			}

			let rsprint_name = CString::new("rsprint").unwrap();
			MIR_load_external(mctx, rsprint_name.as_ptr(), rsprint as *mut c_void);

			MIR_link(mctx, Some(MIR_set_interp_interface), Some(import_resolver));

			let mut walker = (*to_link).items.head;

			loop {
				if walker.is_null() {
					break;
				}

				if (*walker).item_type == MIR_item_type_t_MIR_func_item {
					let cname = CStr::from_ptr((*(*walker).u.func).name);

					if cname.to_str().is_ok_and(|n| n == "_main") {
						let generated = MIR_gen(mctx, 0, walker);
						let callable = std::mem::transmute::<_, fn() -> i32>(generated);
						let integer = callable();
						assert_eq!(integer, 999);
					}
				}

				walker = (*walker).item_link.next;
			}
		}
	}
}
