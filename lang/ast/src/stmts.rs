#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use salite_macros::{CtorCall, FieldCall};
use salite_traits::SpannedNode;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, FieldCall, CtorCall)]
pub struct ReturnStmt {
    #[exclude]
    span: Span,
    exprlist: ExprList,
}

impl Node for ReturnStmt {
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::Return(self.clone()))
    }
}

impl SpannedNode for ReturnStmt {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        match self {
            Stmt::Break(node) => Some(Stmt::Break(node.clone())),
            Stmt::Call(node) => node.as_stmt(),
            Stmt::Do(node) => node.as_stmt(),
            Stmt::FunctionAssign(node) => node.as_stmt(),
            Stmt::GenericFor(node) => node.as_stmt(),
            Stmt::If(node) => node.as_stmt(),
            Stmt::LocalAssign(node) => node.as_stmt(),
            Stmt::LocalFunction(node) => node.as_stmt(),
            Stmt::NumericFor(node) => node.as_stmt(),
            Stmt::Return(node) => node.as_stmt(),
            Stmt::Repeat(node) => node.as_stmt(),
            Stmt::While(node) => node.as_stmt(),
            Stmt::TypeDeclaration(node) => node.as_stmt(),
            Stmt::VarAssign(node) => node.as_stmt(),
        }
    }
}

impl SpannedNode for Stmt {
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

impl SpannedNode for ElseIfClause {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::If(self.clone()))
    }
}

impl SpannedNode for IfStmt {
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

impl SpannedNode for Block {
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

impl SpannedNode for FunctionAssignName {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::FunctionAssign(self.clone()))
    }
}

impl SpannedNode for FunctionAssign {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::LocalFunction(self.clone()))
    }
}

impl SpannedNode for LocalFunction {
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

impl SpannedNode for VarAssignName {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::VarAssign(self.clone()))
    }
}

impl SpannedNode for VarAssign {
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

impl SpannedNode for LocalAssignName {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::LocalAssign(self.clone()))
    }
}

impl SpannedNode for LocalAssign {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::Do(self.clone()))
    }
}

impl SpannedNode for DoStmt {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::While(self.clone()))
    }
}

impl SpannedNode for WhileStmt {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::Repeat(self.clone()))
    }
}

impl SpannedNode for RepeatStmt {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::NumericFor(self.clone()))
    }
}

impl SpannedNode for NumericFor {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::GenericFor(self.clone()))
    }
}

impl SpannedNode for GenericFor {
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
    fn as_expr(&self) -> Option<Expr> {
        None
    }

    fn as_stmt(&self) -> Option<Stmt> {
        Some(Stmt::TypeDeclaration(self.clone()))
    }
}

impl SpannedNode for TypeDeclaration {
    fn span(&self) -> Span {
        self.span
    }
}
