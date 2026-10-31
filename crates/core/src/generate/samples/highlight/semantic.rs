use std::collections::HashSet;

use full_moon::ast::luau::{ExportedTypeDeclaration, IndexedTypeInfo, TypeDeclaration, TypeInfo};
use full_moon::ast::{
	Call, FunctionBody, FunctionCall, FunctionDeclaration, GenericFor, Index, LastStmt,
	LocalAssignment, LocalFunction, NumericFor, Parameter, Prefix, Suffix, Var, VarExpression,
};
use full_moon::tokenizer::TokenReference;
use full_moon::visitors::Visitor;

use super::globals::global;
use super::roles::Roles;
use super::ty::Ty;
use crate::api::code::CodeTone;
use crate::generate::docs::ApiNames;

pub struct Semantic<'a, 'b> {
	names: &'b ApiNames<'a>,
	pub roles: Roles,
	locals: HashSet<String>,
}

fn text(token: &TokenReference) -> String {
	token.token().to_string()
}

const fn call_tone(called: bool) -> CodeTone {
	if called {
		CodeTone::Function
	} else {
		CodeTone::Plain
	}
}

impl<'a, 'b> Semantic<'a, 'b> {
	pub fn new(names: &'b ApiNames<'a>) -> Self {
		Self {
			names,
			roles: Roles::default(),
			locals: HashSet::new(),
		}
	}

	fn name(&mut self, token: &TokenReference, called: bool) -> Ty {
		let name = text(token);
		let found = if self.locals.contains(&name) {
			None
		} else {
			global(self.names, &name)
		};
		if let Some((tone, ty)) = found {
			self.roles.mark(token, tone);
			ty
		} else {
			self.roles.mark(token, call_tone(called));
			Ty::Unknown
		}
	}

	fn member(&mut self, ty: Ty, token: &TokenReference, called: bool) -> Ty {
		let name = text(token);
		let (tone, next) = match ty {
			Ty::EnumRoot if self.names.enums.contains_key(&name) => (CodeTone::Type, Ty::Enum),
			Ty::Enum if !called => (CodeTone::Constant, Ty::Unknown),
			_ => (call_tone(called), Ty::Unknown),
		};
		self.roles.mark(token, tone);
		next
	}

	fn chain<'s>(&mut self, prefix: &Prefix, suffixes: impl Iterator<Item = &'s Suffix>) {
		let suffixes: Vec<&Suffix> = suffixes.collect();
		let called = |index: usize| {
			matches!(
				suffixes.get(index),
				Some(Suffix::Call(Call::AnonymousCall(_)))
			)
		};
		let mut ty = match prefix {
			Prefix::Name(token) => self.name(token, called(0)),
			_ => Ty::Unknown,
		};
		for (index, suffix) in suffixes.iter().enumerate() {
			ty = match suffix {
				Suffix::Index(Index::Dot { name, .. }) => self.member(ty, name, called(index + 1)),
				Suffix::Call(Call::MethodCall(method)) => {
					self.roles.mark(method.name(), CodeTone::Function);
					Ty::Unknown
				}
				_ => Ty::Unknown,
			};
		}
	}

	fn declare(&mut self, token: &TokenReference) {
		self.locals.insert(text(token));
	}

	fn mark_type(&mut self, token: &TokenReference) {
		self.roles.mark(token, CodeTone::Type);
	}
}

impl Visitor for Semantic<'_, '_> {
	fn visit_function_call(&mut self, node: &FunctionCall) {
		self.chain(node.prefix(), node.suffixes());
	}

	fn visit_var_expression(&mut self, node: &VarExpression) {
		self.chain(node.prefix(), node.suffixes());
	}

	fn visit_var(&mut self, node: &Var) {
		if let Var::Name(token) = node {
			self.name(token, false);
		}
	}

	fn visit_local_assignment_end(&mut self, node: &LocalAssignment) {
		for token in node.names() {
			self.declare(token);
		}
	}

	fn visit_function_body(&mut self, node: &FunctionBody) {
		for parameter in node.parameters() {
			if let Parameter::Name(token) = parameter {
				self.declare(token);
			}
		}
	}

	fn visit_generic_for(&mut self, node: &GenericFor) {
		for token in node.names() {
			self.declare(token);
		}
	}

	fn visit_numeric_for(&mut self, node: &NumericFor) {
		self.declare(node.index_variable());
	}

	fn visit_local_function(&mut self, node: &LocalFunction) {
		self.roles.mark(node.name(), CodeTone::Function);
		self.declare(node.name());
	}

	fn visit_function_declaration(&mut self, node: &FunctionDeclaration) {
		let name = node.name();
		if let Some(token) = name.method_name().or_else(|| name.names().iter().last()) {
			self.roles.mark(token, CodeTone::Function);
		}
	}

	fn visit_type_info(&mut self, node: &TypeInfo) {
		match node {
			TypeInfo::Basic(token) => self.mark_type(token),
			TypeInfo::Generic { base, .. } => self.mark_type(base),
			TypeInfo::Module { type_info, .. } => {
				if let IndexedTypeInfo::Basic(token)
				| IndexedTypeInfo::Generic { base: token, .. } = type_info.as_ref()
				{
					self.mark_type(token);
				}
			}
			_ => {}
		}
	}

	fn visit_type_declaration(&mut self, node: &TypeDeclaration) {
		self.roles.mark(node.type_token(), CodeTone::Keyword);
		self.mark_type(node.type_name());
	}

	fn visit_exported_type_declaration(&mut self, node: &ExportedTypeDeclaration) {
		self.roles.mark(node.export_token(), CodeTone::Keyword);
	}

	fn visit_last_stmt(&mut self, node: &LastStmt) {
		if let LastStmt::Continue(token) = node {
			self.roles.mark(token, CodeTone::Keyword);
		}
	}
}
