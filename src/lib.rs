
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
	fn field_info() -> Vec<FieldInfo>;

	/// Get all field info (including padding)
	fn all_field_info() -> Vec<FieldInfo>;

	/// Iterate fields (excluding padding)
	fn iterate_fields(&self, f: impl FnMut(&str, &dyn std::any::Any));

	/// Iterate all fields (including padding)
	fn iterate_all_fields(&self, f: impl FnMut(&str, &dyn std::any::Any));
}
