#[derive(Debug)]
pub struct SelectStruct {
    pub columns : Vec<QualifiedIdentifierT>,
    pub table : AliasedIdentifierT,
    pub joins : Vec<JoinSpecificationT>,
    pub filter : Option<Vec<LogicalExpressionT>>
}

#[derive(Debug)]
pub struct QualifiedIdentifierT {
    pub name : String,
    pub qualifier : Option<String>
}

#[derive(Debug)]
pub struct AliasedIdentifierT {
    pub name : QualifiedIdentifierT,
    pub alias : Option<String>
}

#[derive(Debug)]
pub struct JoinSpecificationT {
    pub table : AliasedIdentifierT,
    pub on : LogicalExpressionT,
    pub typ : JoinType
}

#[derive(Debug)]
pub enum ExpressionTermT {
    Identifier(QualifiedIdentifierT),
    Number(i64),
    StringLiteral(String)
}

#[derive(Debug)]
pub enum LogicalExpressionT {
    Binary(TwoSidedExpressionT),
    Unary(UnaryExpressionT)
}

#[derive(Debug)]
pub struct UnaryExpressionT {
    pub v1 : ExpressionTermT,
    pub operator : UnaryOperator
}

#[derive(Debug)]
pub struct TwoSidedExpressionT {
    pub v1 : ExpressionTermT,
    pub v2 : ExpressionTermT,
    pub operator : ComparisionOperator
}

#[derive(Debug)]
pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
    Cross,
    None
}

#[derive(Debug)]
pub enum ComparisionOperator {
    Eq,
    Like
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    NotNull,
    IsNull
}

#[derive(Debug)]
pub enum LogicalOperator {
    AND,
    OR
}
