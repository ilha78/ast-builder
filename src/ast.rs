///
/// Enum representing all types of operations on unary expressions
/// This is also used as a field to UnaryExpr
/// 
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Forward,
    Back,
    Left,
    Right,
    Setpencolor,
    Turn,
    Setheading,
    Setx,
    Sety,
}

///
/// Enum representing all types of operations on binary expressions
/// This is also used as a field to BinaryExpr
///
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Make,
    Addassign,
    If,
    While,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Lt,
    And,
    Or,
    Func(String),
}

///
/// Enum representing all types of expressions in the AST as 'Node'
/// Some expressions represent terminal nodes such as literals and variables
/// whereas other expressions represent internal nodes such as operators
/// 
/// Note that 'if' and 'while' statements are broken into condition-body
/// where the lhs stores the condition and rhs stores the body
/// 
/// Note that 'defined function calls' are broken into argument-body
/// where the lhs stores the arguments and rhs references the body of the function
///
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Node {
    Literal(String),
    Variable(String),
    Xcor,
    Ycor,
    Heading,
    Color,
    Penup,
    Pendown,
    UnaryExpr {
        op: UnaryOp,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Body(Vec<Node>),
    Empty,
    Newline,
    Caller {
        name: String,
        args: Box<Node>,
    },
}

///
/// Enum representing the possible leaf nodes
/// In the AST, this is either a Literal or Variable
///
#[derive(Debug, Clone)]
pub enum Leaf {
    Literal,
    Variable,
}
