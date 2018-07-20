#[derive(Debug)]
pub struct SelectStruct {
    pub columns : Vec<QualifiedIdentifierT>,
    pub table : AliasedIdentifierT,
    pub joins : Vec<JoinSpecificationT>,
    pub filter : Option<Vec<TwoSidedExpressionT>>
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
    pub on : TwoSidedExpressionT,
    pub typ : JoinType
}

#[derive(Debug)]
pub enum ExpressionTermT {
    Identifier(QualifiedIdentifierT),
    Number(i64),
    StringLiteral(String)
}

#[derive(Debug)]
pub struct TwoSidedExpressionT {
    pub v1 : ExpressionTermT,
    pub v2 : ExpressionTermT,
    pub operator : Operator
}

#[derive(Debug)]
pub enum JoinType {
    INNER,
    LEFT_OUTER,
    RIGHT_OUTER,
    FULL_OUTER,
    CROSS,
    NONE
}

#[derive(Debug)]
pub enum Operator {
    EQ
}

#[derive(Debug)]
pub enum LogicalOperator {
    AND,
    OR
}
