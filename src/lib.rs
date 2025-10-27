use std::vec::IntoIter;

/// Field information structure
#[derive(Debug, Clone)]
pub struct FieldInfo {
	pub name: &'static str,
	pub type_id: std::any::TypeId,
	pub type_name: &'static str,
	pub size: usize,
	pub is_padding: bool,
}

/// FFIStruct trait
pub trait FFIStruct {
	/// Get struct alignment
	fn alignment() -> usize;

	/// Get field info (excluding padding)
	fn iter_members() -> IntoIter<FieldInfo>;

	/// Get all field info (including padding)
	fn iter_all_members() -> IntoIter<FieldInfo>;
}
