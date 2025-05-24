pub enum DemoNode {
    /// Node with single input.
    /// Displays the value of the input.
    Sink,

    /// Value node with a single output.
    /// The value is editable in UI.
    Number(f64),

    /// Value node with a single output.
    String(String),

    /// Converts URI to Image
    ShowImage(String),

    /// Expression node with a single output.
    /// It has number of inputs equal to number of variables in the expression.
    ExprNode(ExprNode),
}

impl DemoNode {
    const fn name(&self) -> &str {
        match self {
            DemoNode::Sink => "Sink",
            DemoNode::Number(_) => "Number",
            DemoNode::String(_) => "String",
            DemoNode::ShowImage(_) => "ShowImage",
            DemoNode::ExprNode(_) => "ExprNode",
        }
    }

    pub fn number_out(&self) -> f64 {
        match self {
            DemoNode::Number(value) => *value,
            DemoNode::ExprNode(expr_node) => expr_node.eval(),
            _ => unreachable!(),
        }
    }

    pub fn number_in(&mut self, idx: usize) -> &mut f64 {
        match self {
            DemoNode::ExprNode(expr_node) => &mut expr_node.values[idx - 1],
            _ => unreachable!(),
        }
    }

    pub fn label_in(&mut self, idx: usize) -> &str {
        match self {
            DemoNode::ShowImage(_) if idx == 0 => "URL",
            DemoNode::ExprNode(expr_node) => &expr_node.bindings[idx - 1],
            _ => unreachable!(),
        }
    }

    pub fn string_out(&self) -> &str {
        match self {
            DemoNode::String(value) => value,
            _ => unreachable!(),
        }
    }

    pub fn string_in(&mut self) -> &mut String {
        match self {
            DemoNode::ShowImage(uri) => uri,
            DemoNode::ExprNode(expr_node) => &mut expr_node.text,
            _ => unreachable!(),
        }
    }

    pub fn expr_node(&mut self) -> &mut ExprNode {
        match self {
            DemoNode::ExprNode(expr_node) => expr_node,
            _ => unreachable!(),
        }
    }
}

pub struct ExprNode {
    pub text: String,
    pub bindings: Vec<String>,
    pub values: Vec<f64>,
    pub expr: Expr,
}

impl ExprNode {
    pub fn new() -> Self {
        ExprNode {
            text: "0".to_string(),
            bindings: Vec::new(),
            values: Vec::new(),
            expr: Expr::Val(0.0),
        }
    }

    pub fn eval(&self) -> f64 {
        self.expr.eval(&self.bindings, &self.values)
    }
}

enum UnOp {
    Pos,
    Neg,
}

enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expr {
    Var(String),
    Val(f64),
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

impl Expr {
    fn eval(&self, bindings: &[String], args: &[f64]) -> f64 {
        let binding_index =
            |name: &str| bindings.iter().position(|binding| binding == name).unwrap();

        match self {
            Expr::Var(name) => args[binding_index(name)],
            Expr::Val(value) => *value,
            Expr::UnOp { op, expr } => match op {
                UnOp::Pos => expr.eval(bindings, args),
                UnOp::Neg => -expr.eval(bindings, args),
            },
            Expr::BinOp { lhs, op, rhs } => match op {
                BinOp::Add => lhs.eval(bindings, args) + rhs.eval(bindings, args),
                BinOp::Sub => lhs.eval(bindings, args) - rhs.eval(bindings, args),
                BinOp::Mul => lhs.eval(bindings, args) * rhs.eval(bindings, args),
                BinOp::Div => lhs.eval(bindings, args) / rhs.eval(bindings, args),
            },
        }
    }

    pub fn extend_bindings(&self, bindings: &mut Vec<String>) {
        match self {
            Expr::Var(name) => {
                if !bindings.contains(name) {
                    bindings.push(name.clone());
                }
            }
            Expr::Val(_) => {}
            Expr::UnOp { expr, .. } => {
                expr.extend_bindings(bindings);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                lhs.extend_bindings(bindings);
                rhs.extend_bindings(bindings);
            }
        }
    }
}

impl syn::parse::Parse for UnOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![+]) {
            input.parse::<syn::Token![+]>()?;
            Ok(UnOp::Pos)
        } else if lookahead.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            Ok(UnOp::Neg)
        } else {
            Err(lookahead.error())
        }
    }
}

impl syn::parse::Parse for BinOp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![+]) {
            input.parse::<syn::Token![+]>()?;
            Ok(BinOp::Add)
        } else if lookahead.peek(syn::Token![-]) {
            input.parse::<syn::Token![-]>()?;
            Ok(BinOp::Sub)
        } else if lookahead.peek(syn::Token![*]) {
            input.parse::<syn::Token![*]>()?;
            Ok(BinOp::Mul)
        } else if lookahead.peek(syn::Token![/]) {
            input.parse::<syn::Token![/]>()?;
            Ok(BinOp::Div)
        } else {
            Err(lookahead.error())
        }
    }
}

impl syn::parse::Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let lhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let expr = content.parse::<Expr>()?;
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        // } else if lookahead.peek(syn::LitFloat) {
        //     let lit = input.parse::<syn::LitFloat>()?;
        //     let value = lit.base10_parse::<f64>()?;
        //     let expr = Expr::Val(value);
        //     if input.is_empty() {
        //         return Ok(expr);
        //     }
        //     lhs = expr;
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::Val(value);
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            let expr = Expr::Var(ident.to_string());
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else {
            let unop = input.parse::<UnOp>()?;

            return Self::parse_with_unop(unop, input);
        }

        let binop = input.parse::<BinOp>()?;

        Self::parse_binop(Box::new(lhs), binop, input)
    }
}

impl Expr {
    fn parse_with_unop(op: UnOp, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let lhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let expr = Expr::UnOp {
                op,
                expr: Box::new(content.parse::<Expr>()?),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::LitFloat) {
            let lit = input.parse::<syn::LitFloat>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Val(value)),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Val(value)),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            let expr = Expr::UnOp {
                op,
                expr: Box::new(Expr::Var(ident.to_string())),
            };
            if input.is_empty() {
                return Ok(expr);
            }
            lhs = expr;
        } else {
            return Err(lookahead.error());
        }

        let op = input.parse::<BinOp>()?;

        Self::parse_binop(Box::new(lhs), op, input)
    }

    fn parse_binop(lhs: Box<Expr>, op: BinOp, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let rhs;
        if lookahead.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            rhs = Box::new(content.parse::<Expr>()?);
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::LitFloat) {
            let lit = input.parse::<syn::LitFloat>()?;
            let value = lit.base10_parse::<f64>()?;
            rhs = Box::new(Expr::Val(value));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::LitInt) {
            let lit = input.parse::<syn::LitInt>()?;
            let value = lit.base10_parse::<f64>()?;
            rhs = Box::new(Expr::Val(value));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            rhs = Box::new(Expr::Var(ident.to_string()));
            if input.is_empty() {
                return Ok(Expr::BinOp { lhs, op, rhs });
            }
        } else {
            return Err(lookahead.error());
        }

        let next_op = input.parse::<BinOp>()?;

        if let (BinOp::Add | BinOp::Sub, BinOp::Mul | BinOp::Div) = (&op, &next_op) {
            let rhs = Self::parse_binop(rhs, next_op, input)?;
            Ok(Self::BinOp {
                lhs,
                op,
                rhs: Box::new(rhs),
            })
        } else {
            let lhs = Self::BinOp { lhs, op, rhs };
            Self::parse_binop(Box::new(lhs), next_op, input)
        }
    }
}
