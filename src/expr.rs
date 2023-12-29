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
	fn visit_pyawait_expr(&self, pyawait: &PyAwait<T>) -> T;
	fn visit_pyyield_expr(&self, pyyield: &PyYield<T>) -> T;
	fn visit_yieldfrom_expr(&self, yieldfrom: &YieldFrom<T>) -> T;
	fn visit_compare_expr(&self, compare: &Compare<T>) -> T;
	fn visit_call_expr(&self, call: &Call<T>) -> T;
	fn visit_formattedvalue_expr(&self, formattedvalue: &FormattedValue<T>) -> T;
	fn visit_joinedstr_expr(&self, joinedstr: &JoinedStr<T>) -> T;
	fn visit_constant_expr(&self, constant: &Constant) -> T;
	fn visit_attribute_expr(&self, attribute: &Attribute<T>) -> T;
	fn visit_subscript_expr(&self, subscript: &Subscript<T>) -> T;
	fn visit_starred_expr(&self, starred: &Starred<T>) -> T;
	fn visit_name_expr(&self, name: &Name) -> T;
	fn visit_list_expr(&self, list: &List<T>) -> T;
	fn visit_tuple_expr(&self, tuple: &Tuple<T>) -> T;
	fn visit_slice_expr(&self, slice: &Slice<T>) -> T;
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

struct PyAwait<T> {
	value: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for PyAwait<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_pyawait_expr(self);
	}
}

struct PyYield<T> {
	value: Option<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for PyYield<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_pyyield_expr(self);
	}
}

struct YieldFrom<T> {
	value: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for YieldFrom<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_yieldfrom_expr(self);
	}
}

struct Compare<T> {
	left: Box<dyn Expr<T>>,
	ops: Vec<Token>,
	comparators: Vec<Box<dyn Expr>>,
}

impl<T> Expr<T> for Compare<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_compare_expr(self);
	}
}

struct Call<T> {
	func: Box<dyn Expr<T>>,
	Args: Vec<Box<dyn Expr<T>>>,
	keywords: Vec<Keyword>,
}

impl<T> Expr<T> for Call<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_call_expr(self);
	}
}

struct FormattedValue<T> {
	value: Box<dyn Expr<T>>,
	conversion: Int,
	format_spec: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for FormattedValue<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_formattedvalue_expr(self);
	}
}

struct JoinedStr<T> {
	values: Vec<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for JoinedStr<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_joinedstr_expr(self);
	}
}

struct Constant {
	value: Literal,
	kind: Option<String>,
}

impl<T> Expr<T> for Constant {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_constant_expr(self);
	}
}

struct Attribute<T> {
	value: Box<dyn Expr<T>>,
	attr: Identifier,
	ctx: ExprContext,
}

impl<T> Expr<T> for Attribute<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_attribute_expr(self);
	}
}

struct Subscript<T> {
	value: Box<dyn Expr<T>>,
	slice: Box<dyn Expr>,
	ctx: ExprContext,
}

impl<T> Expr<T> for Subscript<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_subscript_expr(self);
	}
}

struct Starred<T> {
	value: Box<dyn Expr<T>>,
	ctx: ExprContext,
}

impl<T> Expr<T> for Starred<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_starred_expr(self);
	}
}

struct Name {
	id: Identifier,
	ctx: ExprContext,
}

impl<T> Expr<T> for Name {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_name_expr(self);
	}
}

struct List<T> {
	elts: Vec<Box<dyn Expr<T>>>,
	ctx: ExprContext,
}

impl<T> Expr<T> for List<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_list_expr(self);
	}
}

struct Tuple<T> {
	elts: Vec<Box<dyn Expr<T>>>,
	ctx: ExprContext,
}

impl<T> Expr<T> for Tuple<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_tuple_expr(self);
	}
}

struct Slice<T> {
	lower: Option<Box<dyn Expr<T>>>,
	upper: Option<Box<dyn Expr<T>>>,
	step: Option<Box<dyn Expr<T>>>,
}

impl<T> Expr<T> for Slice<T> {
	fn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {
		return visitor.visit_slice_expr(self);
	}
}

