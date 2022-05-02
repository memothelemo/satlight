#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use lunar_macros::{CtorCall, FieldCall};
use lunar_traits::Node;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct ReturnStmt {
    #[exclude]
    span: Span,
    exprlist: ExprList,
}

impl Node for ReturnStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Break(Token),
    Call(Expr),
    Do(DoStmt),
    FunctionAssign(FunctionAssign),
    GenericFor(GenericFor),
    If(IfStmt),
    LocalAssign(LocalAssign),
    LocalFunction(LocalFunction),
    NumericFor(NumericFor),
    Return(ReturnStmt),
    Repeat(RepeatStmt),
    While(WhileStmt),
    TypeDeclaration(TypeDeclaration),
    VarAssign(VarAssign),
}

impl Node for Stmt {
    fn span(&self) -> Span {
        match self {
            Stmt::Break(node) => node.span(),
            Stmt::Call(node) => node.span(),
            Stmt::Do(node) => node.span(),
            Stmt::FunctionAssign(node) => node.span(),
            Stmt::GenericFor(node) => node.span(),
            Stmt::If(node) => node.span(),
            Stmt::LocalAssign(node) => node.span(),
            Stmt::LocalFunction(node) => node.span(),
            Stmt::NumericFor(node) => node.span(),
            Stmt::Return(node) => node.span(),
            Stmt::Repeat(node) => node.span(),
            Stmt::While(node) => node.span(),
            Stmt::VarAssign(node) => node.span(),
            Stmt::TypeDeclaration(node) => node.span(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct ElseIfClause {
    #[exclude]
    span: Span,
    condition: Expr,
    block: Block,
}

impl Node for ElseIfClause {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct IfStmt {
    #[exclude]
    span: Span,
    condition: Expr,
    block: Block,
    elseifs: Vec<ElseIfClause>,
    else_block: Option<Block>,
}

impl Node for IfStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct Block {
    #[exclude]
    span: Span,
    stmts: Vec<Stmt>,
    last_stmt: Option<Box<Stmt>>,
}

impl Node for Block {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionAssignName {
    Property(Box<FunctionAssignName>, Token),
    Method(Box<FunctionAssignName>, Token),
    Name(Token),
}

impl Node for FunctionAssignName {
    fn span(&self) -> Span {
        match self {
            FunctionAssignName::Name(a) => a.span(),
            FunctionAssignName::Property(a, b) => Span::merge(a.span(), b.span()),
            FunctionAssignName::Method(a, b) => Span::merge(a.span(), b.span()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct FunctionAssign {
    #[exclude]
    span: Span,
    name: FunctionAssignName,
    body: FunctionBody,
}

impl Node for FunctionAssign {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct LocalFunction {
    #[exclude]
    span: Span,
    name: Token,
    body: FunctionBody,
}

impl Node for LocalFunction {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum VarAssignName {
    Name(Token),
    Suffixed(Suffixed),
}

impl Node for VarAssignName {
    fn span(&self) -> Span {
        match self {
            VarAssignName::Name(node) => node.span(),
            VarAssignName::Suffixed(node) => node.span(),
        }
    }
}

pub type VarAssignNameList = Vec<VarAssignName>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct VarAssign {
    #[exclude]
    span: Span,
    names: VarAssignNameList,
    exprlist: ExprList,
}

impl Node for VarAssign {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct LocalAssignName {
    #[exclude]
    span: Span,
    name: Token,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    type_info: Option<TypeInfo>,
}

impl Node for LocalAssignName {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct LocalAssign {
    #[exclude]
    span: Span,
    names: Vec<LocalAssignName>,
    exprlist: ExprList,
}

impl LocalAssign {
    pub fn into_segments(&self) -> Vec<(&LocalAssignName, Option<&Expr>)> {
        let mut segments = Vec::new();
        for (id, name) in self.names.iter().enumerate() {
            segments.push((name, self.exprlist.get(id)));
        }
        segments
    }
}

impl Node for LocalAssign {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct DoStmt {
    #[exclude]
    span: Span,
    block: Block,
}

impl Node for DoStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct WhileStmt {
    #[exclude]
    span: Span,
    condition: Expr,
    block: Block,
}

impl Node for WhileStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct RepeatStmt {
    #[exclude]
    span: Span,
    block: Block,
    condition: Expr,
}

impl Node for RepeatStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct NumericFor {
    #[exclude]
    span: Span,
    name: Token,
    start: Box<Expr>,
    end: Box<Expr>,
    step: Option<Expr>,
    block: Block,
}

impl Node for NumericFor {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct GenericFor {
    #[exclude]
    span: Span,
    names: Vec<Token>,
    exprlist: ExprList,
    block: Block,
}

impl Node for GenericFor {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct TypeDeclaration {
    #[exclude]
    span: Span,
    name: Token,
    params: Option<Vec<TypeParameter>>,
    typ: TypeInfo,
}

impl Node for TypeDeclaration {
    fn span(&self) -> Span {
        self.span
    }
}
