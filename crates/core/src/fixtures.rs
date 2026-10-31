use crate::model::{
	ApiType, Capabilities, Class, Member, MemberDetail, Parameter, Security, TypeCategory,
};

pub fn ty(category: TypeCategory, name: &str, optional: bool) -> ApiType {
	ApiType {
		category,
		name: name.to_owned(),
		optional,
		members: None,
	}
}

pub fn parameter(name: &str, ty: ApiType, default: Option<&str>) -> Parameter {
	Parameter {
		name: name.to_owned(),
		ty,
		default: default.map(str::to_owned),
		description: None,
	}
}

pub fn member(name: &str, detail: MemberDetail) -> Member {
	Member {
		name: name.to_owned(),
		history: None,
		tags: Vec::new(),
		security: Security::None,
		thread_safety: "Safe".to_owned(),
		capabilities: Capabilities::default(),
		preferred: None,
		preferred_target: None,
		description: None,
		sample: None,
		detail,
	}
}

pub fn property(name: &str, ty: ApiType, default: Option<&str>) -> Member {
	member(
		name,
		MemberDetail::Property {
			ty,
			category: "Data".to_owned(),
			default: default.map(str::to_owned),
			write_security: Security::None,
			can_load: true,
			can_save: true,
			simulation_access: false,
		},
	)
}

pub fn method(name: &str, parameters: Vec<Parameter>, returns: ApiType) -> Member {
	member(
		name,
		MemberDetail::Method {
			parameters,
			returns,
			return_description: None,
			simulation_access: false,
		},
	)
}

pub fn event(name: &str, parameters: Vec<Parameter>) -> Member {
	member(name, MemberDetail::Event { parameters })
}

pub fn class(name: &str, members: Vec<Member>, superclass: Option<&str>) -> Class {
	Class {
		name: name.to_owned(),
		history: None,
		icon: None,
		superclass: superclass.map(str::to_owned),
		memory_category: "Instances".to_owned(),
		tags: Vec::new(),
		members,
		removed_members: None,
		subclasses: Vec::new(),
		used_by: Vec::new(),
		description: None,
		sample: None,
	}
}
