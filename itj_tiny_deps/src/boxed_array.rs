/// Helper function for creating arrays directly on the heap.
///
/// e.g. consider this code:
///
/// ```
/// let foo = Box::new(["foo", 1_000_000_000]);
/// ```
///
/// This creates a large array *on the stack*, and *then* copies to the heap. As
/// such, the above code will likely result in a stack overflow.
///
/// Using this helper function instead will ensure that the data is created in
/// place on the heap, thereby preventing the stack overflow.
pub fn boxed_array<T: Copy, const N: usize>(val: T) -> Box<[T; N]> {
	let boxed_slice: Box<[T]> = vec![val; N].into_boxed_slice();

	let ptr: *mut [T; N] = Box::into_raw(boxed_slice) as *mut [T; N];

	let out: Box<[T; N]> = unsafe { Box::from_raw(ptr) };

	out
}
