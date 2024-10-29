use crate::ast::*;
use std::{collections::HashMap, iter::Peekable};

type ParseResult<T> = Result<T, String>;

/// 
/// This trait is responsible for handing shared / similar behaviour
/// between different expressions when parsing
/// 
trait ParseProcedure {
    fn do_parse_procedure(
        &self,
        token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
        brackets: &mut i32,
        function_map: &mut HashMap<String, Node>,
    ) -> ParseResult<Node>;
}

///
/// This function will be called to parse te expression where the
/// expression must implement ParseProcedure trait
/// This is made clear with generic T
/// 
fn parse_procedure<T: ParseProcedure>(
    procedure: T,
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    procedure.do_parse_procedure(token_iterator, brackets, function_map)
}

impl ParseProcedure for BinaryOp {
    ///
    /// Consider the different operations of each BinaryOp and process accordingly
    /// This is similar to the Die and Roll exercise from Week 5
    /// 
    fn do_parse_procedure(
        &self,
        token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
        brackets: &mut i32,
        function_map: &mut HashMap<String, Node>,
    ) -> ParseResult<Node> {
        match self {
            BinaryOp::Add => parse_binary(BinaryOp::Add, token_iterator, brackets, function_map),
            BinaryOp::Sub => parse_binary(BinaryOp::Sub, token_iterator, brackets, function_map),
            BinaryOp::Mul => parse_binary(BinaryOp::Mul, token_iterator, brackets, function_map),
            BinaryOp::Div => parse_binary(BinaryOp::Div, token_iterator, brackets, function_map),
            BinaryOp::Make => parse_binary(BinaryOp::Make, token_iterator, brackets, function_map),
            BinaryOp::Addassign => {
                parse_binary(BinaryOp::Addassign, token_iterator, brackets, function_map)
            }
            BinaryOp::Eq => parse_binary(BinaryOp::Eq, token_iterator, brackets, function_map),
            BinaryOp::Ne => parse_binary(BinaryOp::Ne, token_iterator, brackets, function_map),
            BinaryOp::Gt => parse_binary(BinaryOp::Gt, token_iterator, brackets, function_map),
            BinaryOp::Lt => parse_binary(BinaryOp::Lt, token_iterator, brackets, function_map),
            BinaryOp::And => parse_binary(BinaryOp::And, token_iterator, brackets, function_map),
            BinaryOp::Or => parse_binary(BinaryOp::Or, token_iterator, brackets, function_map),
            BinaryOp::If => {
                parse_conditional_binary(BinaryOp::If, token_iterator, brackets, function_map)
            }
            BinaryOp::While => {
                parse_conditional_binary(BinaryOp::While, token_iterator, brackets, function_map)
            }
            BinaryOp::Func(name) => parse_functional_binary(
                BinaryOp::Func(name.to_string()),
                token_iterator,
                brackets,
                function_map,
            ),
        }
    }
}

impl ParseProcedure for UnaryOp {
    ///
    /// Consider the different operations of each UnaryOp and process accordingly
    /// 
    fn do_parse_procedure(
        &self,
        token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
        brackets: &mut i32,
        function_map: &mut HashMap<String, Node>,
    ) -> ParseResult<Node> {
        match self {
            UnaryOp::Forward => {
                parse_unary(UnaryOp::Forward, token_iterator, brackets, function_map)
            }
            UnaryOp::Back => parse_unary(UnaryOp::Back, token_iterator, brackets, function_map),
            UnaryOp::Left => parse_unary(UnaryOp::Left, token_iterator, brackets, function_map),
            UnaryOp::Right => parse_unary(UnaryOp::Right, token_iterator, brackets, function_map),
            UnaryOp::Setpencolor => {
                parse_unary(UnaryOp::Setpencolor, token_iterator, brackets, function_map)
            }
            UnaryOp::Turn => parse_unary(UnaryOp::Turn, token_iterator, brackets, function_map),
            UnaryOp::Setheading => {
                parse_unary(UnaryOp::Setheading, token_iterator, brackets, function_map)
            }
            UnaryOp::Setx => parse_unary(UnaryOp::Setx, token_iterator, brackets, function_map),
            UnaryOp::Sety => parse_unary(UnaryOp::Sety, token_iterator, brackets, function_map),
        }
    }
}

///
/// This function is responsible in parsing a given vector of tokens into
/// an Abstract Syntax Tree. Each time an AST subtree is parsed,
/// we store it chronologically into the ast vector.
/// 
/// Note that each AST subtree is represented by its respective root node
/// which is returned by parse_command and then pushed into ast vector
/// 
/// Note that the WHILE command will trigger this recursively such
/// that it creates its own ast vector which contains its codeblock
/// 
pub fn parse_logo(
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Vec<Node>> {
    let mut ast = vec![];

    loop {
        match parse_command(token_iterator, brackets, function_map) {
            Ok(res) => match res {
                Node::Empty => break,
                Node::Newline => continue,
                _ => ast.push(res),
            },
            Err(e) => return Err(e.to_string()),
        }

        match token_iterator.peek() {
            Some(check_arg) => match check_arg.as_str() {
                "\n" => continue,
                _ => {
                    return Err(String::from("Invalid number of args"));
                }
            },
            None => break,
        }
    }
    Ok(ast)
}

///
/// Parse by the given command
/// Then separate into cases and process accordingly
/// Note that all UnaryOp and BinaryOp cases will call the shared-behaviour
/// function called parse_procedure as they both implement the ParseProcedure trait
/// Thi function is often called recursively to process and expressions of operations / commands
/// before processing the operation / commands itself
/// 
fn parse_command(
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    if let Some(token) = token_iterator.next() {
        match token.as_str() {
            "PENUP" => Ok(Node::Penup),
            "PENDOWN" => Ok(Node::Pendown),
            "HEADING" => Ok(Node::Heading),
            "XCOR" => Ok(Node::Xcor),
            "YCOR" => Ok(Node::Ycor),
            "COLOR" => Ok(Node::Color),
            "\"" => parse_leaf(token_iterator, Leaf::Literal),
            ":" => parse_leaf(token_iterator, Leaf::Variable),

            "FORWARD" => parse_procedure(UnaryOp::Forward, token_iterator, brackets, function_map),
            "BACK" => parse_procedure(UnaryOp::Back, token_iterator, brackets, function_map),
            "LEFT" => parse_procedure(UnaryOp::Left, token_iterator, brackets, function_map),
            "RIGHT" => parse_procedure(UnaryOp::Right, token_iterator, brackets, function_map),
            "SETPENCOLOR" => {
                parse_procedure(UnaryOp::Setpencolor, token_iterator, brackets, function_map)
            }
            "TURN" => parse_procedure(UnaryOp::Turn, token_iterator, brackets, function_map),
            "SETHEADING" => {
                parse_procedure(UnaryOp::Setheading, token_iterator, brackets, function_map)
            }
            "SETX" => parse_procedure(UnaryOp::Setx, token_iterator, brackets, function_map),
            "SETY" => parse_procedure(UnaryOp::Sety, token_iterator, brackets, function_map),

            "MAKE" => parse_procedure(BinaryOp::Make, token_iterator, brackets, function_map),
            "ADDASSIGN" => {
                parse_procedure(BinaryOp::Addassign, token_iterator, brackets, function_map)
            }
            "IF" => parse_procedure(BinaryOp::If, token_iterator, brackets, function_map),
            "WHILE" => parse_procedure(BinaryOp::While, token_iterator, brackets, function_map),
            "EQ" => parse_procedure(BinaryOp::Eq, token_iterator, brackets, function_map),
            "NE" => parse_procedure(BinaryOp::Ne, token_iterator, brackets, function_map),
            "GT" => parse_procedure(BinaryOp::Gt, token_iterator, brackets, function_map),
            "LT" => parse_procedure(BinaryOp::Lt, token_iterator, brackets, function_map),
            "AND" => parse_procedure(BinaryOp::And, token_iterator, brackets, function_map),
            "OR" => parse_procedure(BinaryOp::Or, token_iterator, brackets, function_map),
            "+" => parse_procedure(BinaryOp::Add, token_iterator, brackets, function_map),
            "-" => parse_procedure(BinaryOp::Sub, token_iterator, brackets, function_map),
            "*" => parse_procedure(BinaryOp::Mul, token_iterator, brackets, function_map),
            "/" => parse_procedure(BinaryOp::Div, token_iterator, brackets, function_map),

            "TO" => {
                *brackets += 1;
                if let Some(func_name) = token_iterator.next() {
                    match parse_procedure(
                        BinaryOp::Func(func_name.to_string()),
                        token_iterator,
                        brackets,
                        function_map,
                    ) {
                        Ok(func_data) => {
                            function_map.insert(func_name, func_data.clone());
                            Ok(func_data)
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err(String::from("No function name2"))
                }
            }
            "END" => {
                if *brackets == 0 {
                    Err(String::from("Invalid function"))
                } else {
                    *brackets -= 1;
                    Ok(Node::Empty)
                }
            }
            "//" => loop {
                if let Some(res) = token_iterator.next() {
                    if res == "\n" {
                        return parse_command(token_iterator, brackets, function_map);
                    }
                }
            },
            "\n" => Ok(Node::Newline),
            "[" => {
                *brackets += 1;
                parse_command(token_iterator, brackets, function_map)
            }
            "]" => {
                if *brackets == 0 {
                    Err(String::from("Invalid brackets"))
                } else {
                    *brackets -= 1;
                    Ok(Node::Empty)
                }
            }
            k => {
                if function_map.contains_key(&k.to_string()) {
                    let mut args = vec![];
                    loop {
                        if let Ok(arg) = parse_command(token_iterator, brackets, function_map) {
                            match arg {
                                Node::Newline => break,
                                _ => {
                                    args.push(arg);
                                    if let Some(token) = token_iterator.peek() {
                                        if token.as_str() == "\n" {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            return Err(String::from("cannot parse arg of defined func"));
                        }
                    }
                    Ok(Node::Caller {
                        name: k.to_string(),
                        args: Box::new(Node::Body(args)),
                    })
                } else {
                    Err(String::from("wrong format"))
                }
            }
        }
    } else {
        Ok(Node::Empty)
    }
}

///
/// parse the unary expression
/// 
fn parse_unary(
    procedure: UnaryOp,
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    if let Ok(expr) = parse_command(token_iterator, brackets, function_map) {
        Ok(Node::UnaryExpr {
            op: procedure,
            child: Box::new(expr),
        })
    } else {
        Err(String::from("Invalid args unary"))
    }
}

///
/// parse the defined function node
/// argument is on lhs as vector of Nodes
/// and body is on rhs
/// 
fn parse_functional_binary(
    procedure: BinaryOp,
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    let mut args = vec![];
    loop {
        if let Ok(arg) = parse_command(token_iterator, brackets, function_map) {
            match arg {
                Node::Newline => break,
                _ => {
                    args.push(arg);
                    if let Some(token) = token_iterator.peek() {
                        if token.as_str() == "\n" {
                            break;
                        }
                    }
                }
            }
        } else {
            return Err(String::from("cannot parse arg of defined func"));
        }
    }
    match parse_logo(token_iterator, brackets, function_map) {
        Ok(body) => Ok(Node::BinaryExpr {
            op: procedure,
            lhs: Box::new(Node::Body(args)),
            rhs: Box::new(Node::Body(body)),
        }),
        Err(e) => Err(e),
    }
}

///
/// parsing if and while nodes
/// lhs has the condition and rhs has the body
/// 
fn parse_conditional_binary(
    procedure: BinaryOp,
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    if let Ok(cond) = parse_command(token_iterator, brackets, function_map) {
        match parse_logo(token_iterator, brackets, function_map) {
            Ok(body) => Ok(Node::BinaryExpr {
                op: procedure,
                lhs: Box::new(cond),
                rhs: Box::new(Node::Body(body)),
            }),
            Err(e) => Err(e),
        }
    } else {
        Err(String::from("Invalid cond"))
    }
}

/// 
/// parse binary expressions
/// 
fn parse_binary(
    procedure: BinaryOp,
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    brackets: &mut i32,
    function_map: &mut HashMap<String, Node>,
) -> ParseResult<Node> {
    if let Ok(expr1) = parse_command(token_iterator, brackets, function_map) {
        if let Ok(expr2) = parse_command(token_iterator, brackets, function_map) {
            Ok(Node::BinaryExpr {
                op: procedure,
                lhs: Box::new(expr1),
                rhs: Box::new(expr2),
            })
        } else {
            Err(String::from("Invalid Second Arg"))
        }
    } else {
        Err(String::from("Invalid First Arg"))
    }
}

/// 
/// parse literals and variables
/// 
fn parse_leaf(
    token_iterator: &mut Peekable<std::vec::IntoIter<String>>,
    leaf: Leaf,
) -> ParseResult<Node> {
    if let Some(res) = token_iterator.next() {
        match leaf {
            Leaf::Literal => Ok(Node::Literal(res.to_string())),
            Leaf::Variable => Ok(Node::Variable(res.to_string())),
        }
    } else {
        Err(String::from("Invalid arg"))
    }
}
