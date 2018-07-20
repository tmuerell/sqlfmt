#[derive(Debug)]
pub struct SelectStruct {
    pub columns : Vec<QualifiedIdentifierT>,
    pub table : AliasedIdentifierT,
    pub joins : Vec<JoinSpecificationT>,
    pub filter : Option<TwoSidedExpressionT>
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
pub enum ExpressionT {
    Single(QualifiedIdentifierT),
    Sub(TwoSidedExpressionT),
    Combining(CombiningExpressionT)
}

#[derive(Debug)]
pub struct TwoSidedExpressionT {
    pub v1 : QualifiedIdentifierT,
    pub v2 : QualifiedIdentifierT,
    pub operator : Operator
}

#[derive(Debug)]
pub struct CombiningExpressionT {
    pub v1 : Box<ExpressionT>,
    pub v2 : Box<ExpressionT>,
    pub operator : LogicalOperator
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
