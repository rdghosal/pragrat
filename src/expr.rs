use crate::types::{Literal, Token};

trait Visitor<T> {
	fn visit_boolop_expr(&self, boolop: &BoolOp<T>) -> T;
	fn visit_namedexpr_expr(&self, namedexpr: &NamedExpr<T>) -> T;
	fn visit_binop_expr(&self, binop: &BinOp<T>) -> T;
	fn visit_unaryop_expr(&self, unaryop: &UnaryOp<T>) -> T;
	fn visit_lambda_expr(&self, lambda: &Lambda<T>) -> T;
	fn visit_ifexp_expr(&self, ifexp: &IfExp<T>) -> T;
	fn visit_dict_expr(&self, dict: &Dict<T>) -> T;
	fn visit_set_expr(&self, set: &Set<T>) -> T;
	fn visit_listcomp_expr(&self, listcomp: &ListComp<T>) -> T;
	fn visit_setcomp_expr(&self, setcomp: &SetComp<T>) -> T;
	fn visit_dictcomp_expr(&self, dictcomp: &DictComp<T>) -> T;
	fn visit_generatorexp_expr(&self, generatorexp: &GeneratorExp<T>) -> T;
}

trait Expr<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;
}

struct BoolOp<T> {
	op: Token,
	values: Vec<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for BoolOp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_boolop_expr(self);
	}
}

struct NamedExpr<T> {
	target: Box<dyn Expr<T>>,
	value: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for NamedExpr<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_namedexpr_expr(self);
	}
}

struct BinOp<T> {
	left: Box<dyn Expr<T>>,
	op: Token,
	right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for BinOp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_binop_expr(self);
	}
}

struct UnaryOp<T> {
	op: Token,
	operand: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for UnaryOp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_unaryop_expr(self);
	}
}

struct Lambda<T> {
	args: Arguments,
	body: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Lambda<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_lambda_expr(self);
	}
}

struct IfExp<T> {
	test: Box<dyn Expr<T>>,
	body: Box<dyn Expr<T>>,
	orelse: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for IfExp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_ifexp_expr(self);
	}
}

struct Dict<T> {
	keys: Vec<Box<dyn Expr<T>>>,
	values: Vec<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for Dict<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_dict_expr(self);
	}
}

struct Set<T> {
	elts: Vec<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for Set<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_set_expr(self);
	}
}

struct ListComp<T> {
	elt: Box<dyn Expr<T>>,
	generators: Vec<Comprehension>,
}

impl<T> Expr<T> for ListComp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_listcomp_expr(self);
	}
}

struct SetComp<T> {
	elt: Box<dyn Expr<T>>,
	generators: Vec<Comprehension>,
}

impl<T> Expr<T> for SetComp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_setcomp_expr(self);
	}
}

struct DictComp<T> {
	key: Box<dyn Expr<T>>,
	value: Box<dyn Expr<T>>,
	generators: Vec<Comprehension>,
}

impl<T> Expr<T> for DictComp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_dictcomp_expr(self);
	}
}

struct GeneratorExp<T> {
	key: Box<dyn Expr<T>>,
	value: Box<dyn Expr<T>>,
	generators: Vec<Comprehension>,
}

impl<T> Expr<T> for GeneratorExp<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_generatorexp_expr(self);
	}
}

