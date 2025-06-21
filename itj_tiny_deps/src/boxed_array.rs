/// Helper function for creating arrays directly on the heap.
///
/// e.g. consider this code:
///
/// ```should_panic
/// let foo = Box::new(["foo"; 10_000_000]);
/// ```
///
/// This creates a large array *on the stack*, and *then* copies to the heap. As
/// such, the above code is expected to result in a stack overflow.
///
/// Using this helper function, as below, avoids the overflow by ensuring that
/// the data is created in place on the heap.
///
/// ```
/// use itj_tiny_deps::boxed_array;
/// let foo: Box<[_; 10_000_000]> = boxed_array![123; 10_000_000];
/// ```
#[macro_export]
macro_rules! boxed_array {
	($val:expr ; $len:expr) => {{
		fn boxed_array2<T: Copy, const N: usize>(val: Vec<T>) -> Box<[T; N]> {
			assert_eq!(val.len(), N);
			let boxed_slice: ::std::boxed::Box<[T]> = val.into_boxed_slice();

			let ptr: *mut [T; N] = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; N];

			let out: ::std::boxed::Box<[T; N]> = unsafe { ::std::boxed::Box::from_raw(ptr) };

			out
		}

		boxed_array2(vec![$val; $len])
	}};
}
