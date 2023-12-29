use crate::types::{Literal, Token};

trait Visitor<T> {
	fn visit_binary_expr(&self, binary: &Binary<T>) -> T;
	fn visit_grouping_expr(&self, grouping: &Grouping<T>) -> T;
	fn visit_constant_expr(&self, constant: &Constant) -> T;
	fn visit_unary_expr(&self, unary: &Unary<T>) -> T;
}

trait Expr<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;
}

struct Binary<T> {
	left: Box<dyn Expr<T>>,
	operator: Token,
	right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Binary<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_binary_expr(self);
	}
}

struct Grouping<T> {
	expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Grouping<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_grouping_expr(self);
	}
}

struct Constant {
	value: Literal,
}

impl<T> Expr<T> for Constant {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_constant_expr(self);
	}
}

struct Unary<T> {
	operator: Token,
	right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Unary<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_unary_expr(self);
	}
}

