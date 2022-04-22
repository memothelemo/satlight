#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use lunar_macros::PropertyGetter;
use crate::Node;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct ReturnStmt {
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
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct ElseIfClause {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct IfStmt {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct Block {
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
            FunctionAssignName::Property(a, b) => Span::from_two_spans(a.span(), b.span()),
            FunctionAssignName::Method(a, b) => Span::from_two_spans(a.span(), b.span()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct FunctionAssign {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct LocalFunction {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct VarAssign {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct LocalAssignName {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct LocalAssign {
    span: Span,
    names: Vec<LocalAssignName>,
    exprlist: ExprList,
}

impl Node for LocalAssign {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct DoStmt {
    span: Span,
    block: Block,
}

impl Node for DoStmt {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct WhileStmt {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct RepeatStmt {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct NumericFor {
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
#[derive(Debug, Clone, PartialEq, PropertyGetter)]
pub struct GenericFor {
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
