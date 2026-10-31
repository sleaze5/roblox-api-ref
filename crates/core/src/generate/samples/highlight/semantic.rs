use std::collections::HashMap;

use full_moon::ast::luau::{ExportedTypeDeclaration, IndexedTypeInfo, TypeDeclaration, TypeInfo};
use full_moon::ast::{
	Assignment, Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration,
	Index, LastStmt, LocalAssignment, LocalFunction, Parameter, Prefix, Suffix, Var,
	VarExpression,
};
use full_moon::tokenizer::TokenReference;
use full_moon::visitors::Visitor;

use super::calls::{
	CLASS_NAME_METHODS, CONNECTIONS, RETURNS_NAMED_CLASS, arguments, string_argument,
};
use super::globals::global;
use super::roles::{Roles, position};
use super::ty::{Ty, find_member, is_service, of_annotation, of_api_type};
use crate::api::code::CodeTone;
use crate::generate::docs::ApiNames;
use rapi_routes::MemberKind;

use crate::model::DocLink;

pub struct Semantic<'a, 'b> {
	names: &'b ApiNames<'a>,
	pub roles: Roles,
	scope: HashMap<String, Ty>,
	bound: HashMap<usize, Ty>,
}

fn text(token: &TokenReference) -> String {
	token.token().to_string()
}

impl<'a, 'b> Semantic<'a, 'b> {
	pub fn new(names: &'b ApiNames<'a>) -> Self {
		Self {
			names,
			roles: Roles::default(),
			scope: HashMap::new(),
			bound: HashMap::new(),
		}
	}

	fn name(&mut self, token: &TokenReference, called: bool, mark: bool) -> Ty {
		let name = text(token);
		if let Some(ty) = self.scope.get(&name) {
			if mark && called {
				self.roles.mark(token, CodeTone::Function, None);
			}
			return ty.clone();
		}
		let Some(found) = global(self.names, &name) else {
			if mark && called {
				self.roles.mark(token, CodeTone::Function, None);
			}
			return Ty::Unknown;
		};
		if mark {
			let tone = match &found.ty {
				Ty::Static(_) => CodeTone::Type,
				_ if found.builtin => CodeTone::Constant,
				_ => CodeTone::Plain,
			};
			self.roles.mark(token, tone, found.link);
		}
		found.ty
	}

	fn member(&mut self, ty: &Ty, token: &TokenReference, called: bool, mark: bool) -> Ty {
		let name = text(token);
		let fallback = if called {
			CodeTone::Function
		} else {
			CodeTone::Plain
		};
		let (tone, link, next) = match ty {
			Ty::Object(class_name) => match find_member(self.names, class_name, &name) {
				Some(member) => {
					let owner = self
						.names
						.member_link(class_name, &name)
						.unwrap_or(DocLink::Class {
							name: class_name.clone(),
						});
					let tone = if member.kind() == MemberKind::Method {
						CodeTone::Function
					} else {
						fallback
					};
					let next = match member.kind() {
						MemberKind::Event => Ty::Event(
							member.parameters().iter().map(|parameter| of_api_type(&parameter.ty)).collect(),
						),
						_ => member
							.value_type()
							.or_else(|| member.returns())
							.map_or(Ty::Unknown, of_api_type),
					};
					(tone, Some(owner), next)
				}
				None if !called && is_service(self.names, class_name, &name) => (
					CodeTone::Plain,
					Some(DocLink::Class { name: name.clone() }),
					Ty::Object(name),
				),
				None => (fallback, None, Ty::Unknown),
			},
			Ty::Static(type_name) if called => {
				(CodeTone::Function, None, Ty::Constructor(type_name.clone()))
			}
			Ty::EnumRoot => match self.names.enum_link(&name, None) {
				Some(link) => (CodeTone::Type, Some(link), Ty::Enum(name)),
				None => (fallback, None, Ty::Unknown),
			},
			Ty::Enum(enum_name) => match self.names.enum_link(enum_name, Some(&name)) {
				Some(link) => (CodeTone::Constant, Some(link), Ty::EnumItem(enum_name.clone())),
				None => (fallback, None, Ty::Unknown),
			},
			_ => (fallback, None, Ty::Unknown),
		};
		if mark {
			self.roles.mark(token, tone, link);
		}
		next
	}

	fn class_argument(&mut self, args: &FunctionArgs, mark: bool) -> Option<String> {
		let (token, class_name) = string_argument(args)?;
		if !self.names.classes.contains_key(&class_name) {
			return None;
		}
		if mark {
			self.roles.mark(
				token,
				CodeTone::String,
				Some(DocLink::Class {
					name: class_name.clone(),
				}),
			);
		}
		Some(class_name)
	}

	fn bind_handler(&mut self, args: &FunctionArgs, parameters: &[Ty]) {
		for argument in arguments(args) {
			let Expression::Function(function) = argument else {
				continue;
			};
			for (parameter, ty) in function.body().parameters().iter().zip(parameters) {
				if let Parameter::Name(token) = parameter {
					self.bound.insert(position(token), ty.clone());
				}
			}
		}
	}

	fn method(&mut self, ty: &Ty, name: &TokenReference, args: &FunctionArgs, mark: bool) -> Ty {
		let method_name = text(name);
		if let Ty::Event(parameters) = ty {
			if mark {
				self.roles.mark(name, CodeTone::Function, None);
				if CONNECTIONS.contains(&method_name.as_str()) {
					let parameters = parameters.clone();
					self.bind_handler(args, &parameters);
				}
			}
			return Ty::Unknown;
		}
		let mut next = self.member(ty, name, true, mark);
		if CLASS_NAME_METHODS.contains(&method_name.as_str()) {
			let class_name = self.class_argument(args, mark);
			if RETURNS_NAMED_CLASS.contains(&method_name.as_str())
				&& let Some(class_name) = class_name
			{
				next = Ty::Object(class_name);
			}
		}
		if let Ty::Object(class_name) = ty
			&& method_name == "Clone"
		{
			next = Ty::Object(class_name.clone());
		}
		next
	}

	fn call(&mut self, ty: &Ty, args: &FunctionArgs, mark: bool) -> Ty {
		match ty {
			Ty::Constructor(type_name) if type_name == "Instance" => self
				.class_argument(args, mark)
				.map_or(Ty::Object("Instance".to_owned()), Ty::Object),
			Ty::Constructor(type_name) => Ty::Value(type_name.clone()),
			_ => Ty::Unknown,
		}
	}

	fn chain<'s>(
		&mut self,
		prefix: &Prefix,
		suffixes: impl Iterator<Item = &'s Suffix>,
		mark: bool,
	) -> Ty {
		let suffixes: Vec<&Suffix> = suffixes.collect();
		let called = |index: usize| {
			matches!(
				suffixes.get(index),
				Some(Suffix::Call(Call::AnonymousCall(_)))
			)
		};
		let mut ty = match prefix {
			Prefix::Name(token) => self.name(token, called(0), mark),
			Prefix::Expression(expression) => self.expression(expression),
			_ => Ty::Unknown,
		};
		for (index, suffix) in suffixes.iter().enumerate() {
			ty = match suffix {
				Suffix::Index(Index::Dot { name, .. }) => {
					self.member(&ty, name, called(index + 1), mark)
				}
				Suffix::Call(Call::AnonymousCall(args)) => self.call(&ty, args, mark),
				Suffix::Call(Call::MethodCall(method)) => {
					self.method(&ty, method.name(), method.args(), mark)
				}
				_ => Ty::Unknown,
			};
		}
		ty
	}

	fn expression(&mut self, expression: &Expression) -> Ty {
		match expression {
			Expression::Parentheses { expression, .. } => self.expression(expression),
			Expression::TypeAssertion { type_assertion, .. } => {
				of_annotation(self.names, type_assertion.cast_to())
			}
			Expression::FunctionCall(call) => self.chain(call.prefix(), call.suffixes(), false),
			Expression::Var(Var::Name(token)) => self.name(token, false, false),
			Expression::Var(Var::Expression(var)) => {
				self.chain(var.prefix(), var.suffixes(), false)
			}
			_ => Ty::Unknown,
		}
	}

	fn bind(&mut self, token: &TokenReference, ty: Ty) {
		self.scope.insert(text(token), ty);
	}

	fn mark_type(&mut self, token: &TokenReference) {
		let name = text(token);
		let link = self.names.type_link(&name);
		self.roles.mark(token, CodeTone::Type, link);
	}
}

impl Visitor for Semantic<'_, '_> {
	fn visit_function_call(&mut self, node: &FunctionCall) {
		self.chain(node.prefix(), node.suffixes(), true);
	}

	fn visit_var_expression(&mut self, node: &VarExpression) {
		self.chain(node.prefix(), node.suffixes(), true);
	}

	fn visit_var(&mut self, node: &Var) {
		if let Var::Name(token) = node {
			self.name(token, false, true);
		}
	}

	fn visit_local_assignment_end(&mut self, node: &LocalAssignment) {
		let expressions: Vec<&Expression> = node.expressions().iter().collect();
		let annotations: Vec<_> = node.type_specifiers().collect();
		for (index, token) in node.names().iter().enumerate() {
			let annotated = annotations
				.get(index)
				.copied()
				.flatten()
				.map(|specifier| of_annotation(self.names, specifier.type_info()));
			let ty = match annotated {
				Some(ty) if ty != Ty::Unknown => ty,
				_ => expressions
					.get(index)
					.map_or(Ty::Unknown, |expression| self.expression(expression)),
			};
			self.bind(token, ty);
		}
	}

	fn visit_assignment_end(&mut self, node: &Assignment) {
		let expressions: Vec<&Expression> = node.expressions().iter().collect();
		for (index, var) in node.variables().iter().enumerate() {
			if let Var::Name(token) = var {
				let ty = expressions
					.get(index)
					.map_or(Ty::Unknown, |expression| self.expression(expression));
				self.bind(token, ty);
			}
		}
	}

	fn visit_function_body(&mut self, node: &FunctionBody) {
		for (parameter, specifier) in node.parameters().iter().zip(node.type_specifiers()) {
			let Parameter::Name(token) = parameter else {
				continue;
			};
			let ty = specifier
				.map(|specifier| of_annotation(self.names, specifier.type_info()))
				.filter(|ty| *ty != Ty::Unknown)
				.or_else(|| self.bound.remove(&position(token)))
				.unwrap_or(Ty::Unknown);
			self.bind(token, ty);
		}
	}

	fn visit_local_function(&mut self, node: &LocalFunction) {
		self.roles.mark(node.name(), CodeTone::Function, None);
		self.bind(node.name(), Ty::Unknown);
	}

	fn visit_function_declaration(&mut self, node: &FunctionDeclaration) {
		let name = node.name();
		let last = name.method_name().or_else(|| name.names().iter().last());
		if let Some(token) = last {
			self.roles.mark(token, CodeTone::Function, None);
		}
	}

	fn visit_type_info(&mut self, node: &TypeInfo) {
		match node {
			TypeInfo::Basic(token) => self.mark_type(token),
			TypeInfo::Generic { base, .. } => self.mark_type(base),
			TypeInfo::Module {
				module, type_info, ..
			} => {
				let module_name = text(module);
				if let IndexedTypeInfo::Basic(token) | IndexedTypeInfo::Generic { base: token, .. } =
					type_info.as_ref()
				{
					let link = (module_name == "Enum")
						.then(|| self.names.enum_link(&text(token), None))
						.flatten();
					self.roles.mark(token, CodeTone::Type, link);
				}
			}
			_ => {}
		}
	}

	fn visit_type_declaration(&mut self, node: &TypeDeclaration) {
		self.roles.mark(node.type_token(), CodeTone::Keyword, None);
		self.roles.mark(node.type_name(), CodeTone::Type, None);
	}

	fn visit_exported_type_declaration(&mut self, node: &ExportedTypeDeclaration) {
		self.roles.mark(node.export_token(), CodeTone::Keyword, None);
	}

	fn visit_last_stmt(&mut self, node: &LastStmt) {
		if let LastStmt::Continue(token) = node {
			self.roles.mark(token, CodeTone::Keyword, None);
		}
	}
}
