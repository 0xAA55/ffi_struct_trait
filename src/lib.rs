use std::vec::IntoIter;

/// Field information structure
#[derive(Debug, Clone, Copy)]
pub struct MemberInfo {
	pub size: usize,
	pub offset: usize,
	pub type_name: &'static str,
	pub type_id: std::any::TypeId,
}

/// FFIStruct trait
pub trait FFIStruct: Default {
	/// The original `Xxx_Rust` type
	type RustType;

	/// Get field info (excluding padding)
	fn iter_members(&self) -> IntoIter<(&'static str, MemberInfo)>;

	/// Get all field info (including padding)
	fn iter_all_members(&self) -> IntoIter<(&'static str, MemberInfo)>;
}
