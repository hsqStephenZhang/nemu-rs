use std::rc::Rc;

use crate::{compose::Eat, concat::Concat2};

pub mod compose;
pub mod concat;

use compose::*;

/*
<expr> ::= <decimal-number>
  | <hexadecimal-number>    # "0x"
  | <reg_name>              # "$"
  | "(" <expr> ")"
  | <expr> "+" <expr>
  | <expr> "-" <expr>
  | <expr> "*" <expr>
  | <expr> "/" <expr>
  | <expr> "==" <expr>
  | <expr> "!=" <expr>
  | <expr> "&&" <expr>
  | "*" <expr>              # deref

elimination of left recursion version:
<expr>       ::= <logical_and>
<logical_and>::= <eq> ("&&" <eq>)*
<eq>         ::= <add_sub> (("==" | "!=") <add_sub>)*
<add_sub>    ::= <mul_div> (("+" | "-") <mul_div>)*
<mul_div>    ::= <unary> (("*" | "/") <unary>)*
<unary>      ::= ("*" <unary>) | <primary>
<primary>    ::= <decimal-number>
              | <hexadecimal-number>
              | <reg_name>
              | "(" <expr> ")"
 */

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("Unknown register: {0}")]
    UnknownRegister(String),
    #[error("Invalid pointer dereference at address: {0}")]
    InvalidPointerDereference(i64),
    #[error("Division by zero")]
    DivisionByZero,
}

pub trait Context {
    fn get_reg_value(&self, reg_name: &str) -> Option<i64>;
    fn get_pointer_value(&self, addr: i64) -> Option<i64>;
}

impl Context for () {
    fn get_reg_value(&self, _reg_name: &str) -> Option<i64> {
        None
    }

    fn get_pointer_value(&self, _addr: i64) -> Option<i64> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Decimal(i64),
    Hexadecimal(i64),
    RegName(String),
    Paren(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Deref(Box<Expr>),
}

impl Expr {
    pub fn eval<Ctx: Context>(self: &Self, ctx: &Ctx) -> Result<i64, EvalError> {
        match self {
            Expr::Decimal(value) => Ok(*value),
            Expr::Hexadecimal(value) => Ok(*value),
            Expr::RegName(name) => ctx
                .get_reg_value(name)
                .ok_or_else(|| EvalError::UnknownRegister(name.clone())),
            Expr::Paren(expr) => expr.eval(ctx),
            Expr::Add(lhs, rhs) => Ok(lhs.eval(ctx)? + rhs.eval(ctx)?),
            Expr::Sub(lhs, rhs) => Ok(lhs.eval(ctx)? - rhs.eval(ctx)?),
            Expr::Mul(lhs, rhs) => Ok(lhs.eval(ctx)? * rhs.eval(ctx)?),
            Expr::Div(lhs, rhs) => {
                let right = rhs.eval(ctx)?;
                if right == 0 {
                    Err(EvalError::DivisionByZero)
                } else {
                    Ok(lhs.eval(ctx)? / right)
                }
            }
            Expr::Eq(lhs, rhs) => Ok((lhs.eval(ctx)? == rhs.eval(ctx)?) as i64),
            Expr::Neq(lhs, rhs) => Ok((lhs.eval(ctx)? != rhs.eval(ctx)?) as i64),
            Expr::And(lhs, rhs) => Ok((lhs.eval(ctx)? != 0 && rhs.eval(ctx)? != 0) as i64),
            Expr::Deref(expr) => {
                let addr = expr.eval(ctx)?;
                ctx.get_pointer_value(addr)
                    .ok_or_else(|| EvalError::InvalidPointerDereference(addr))
            }
        }
    }
}

fn digit() -> impl Eat<Output = char> {
    let f = |c: char| c.is_ascii_digit();
    Satisfy(f)
}

fn hex_digit() -> impl Eat<Output = char> {
    let f = |c: char| c.is_ascii_hexdigit();
    Satisfy(f)
}

fn decimal() -> impl Eat<Output = i64> {
    many(digit(), 1).to(|res| {
        let num_str: String = res.into_iter().collect();
        i64::from_str_radix(&num_str, 10).unwrap()
    })
}

fn hex_decimal() -> impl Eat<Output = i64> {
    let prefix = concats((Char('0'), Char('x')));
    let hex = many(hex_digit(), 1).to(|res| {
        let num_str: String = res.into_iter().collect();
        i64::from_str_radix(&num_str, 16).unwrap()
    });
    Concat(prefix, hex).to(|(_, num)| {
        // prefix is "0x", we can ignore it
        num
    })
}

fn reg_name() -> impl Eat<Output = String> {
    let prefix = Char('$');
    let name = many(Satisfy(|c: char| c.is_ascii_alphanumeric() || c == '_'), 1)
        .to(|res| res.into_iter().collect::<String>());
    Concat(prefix, name).to(|(_, name)| name)
}

pub fn parser_expr() -> impl Eat<Output = Expr> {
    let get_expr = || Recursion::new(|_rec, input| parser_expr().eat(input));

    let p_decimal = decimal().to(|res| Expr::Decimal(res));
    let p_hex_decimal = hex_decimal().to(|res| Expr::Hexadecimal(res));
    let p_reg_name = reg_name().to(|name| Expr::RegName(name));
    let p_paren =
        Concat3(Char('('), WS(get_expr()), Char(')')).to(|(_, e, _)| Expr::Paren(Box::new(e)));
    let primary = Rc::new(alt((p_hex_decimal, p_decimal, p_reg_name, p_paren)));

    let deref = Concat2(many(Char('*'), 1), WS(primary.clone()));
    let deref = deref.to(|(chars, e)| {
        let mut res = e;
        for _ in 0..chars.len() {
            res = Expr::Deref(Box::new(res));
        }
        res
    });
    let deref = Rc::new(alt((deref, primary.clone())));

    let mul_div = Concat2(
        WS(deref.clone()),
        many(Concat2(alt((Char('*'), Char('/'))), WS(deref.clone())), 1),
    )
    .to(|(e1, ops)| {
        let mut res = e1;
        for (c, e2) in ops.into_iter() {
            match c {
                '*' => res = Expr::Mul(Box::new(res), Box::new(e2)),
                '/' => res = Expr::Div(Box::new(res), Box::new(e2)),
                _ => unreachable!(),
            }
        }
        res
    });
    let mul_div = Rc::new(alt((mul_div, deref.clone())));

    let add_sub = Concat2(
        WS(mul_div.clone()),
        many(Concat2(alt((Char('+'), Char('-'))), WS(mul_div.clone())), 1),
    )
    .to(|(e1, ops)| {
        let mut res = e1;
        for (c, e2) in ops.into_iter() {
            match c {
                '+' => res = Expr::Add(Box::new(res), Box::new(e2)),
                '-' => res = Expr::Sub(Box::new(res), Box::new(e2)),
                _ => unreachable!(),
            }
        }
        res
    });
    let add_sub = Rc::new(alt((add_sub, mul_div.clone())));

    let eq = Concat2(
        WS(add_sub.clone()),
        many(
            Concat2(
                alt((S("==".to_owned()), S("!=".to_owned()))),
                WS(add_sub.clone()),
            ),
            1,
        ),
    )
    .to(|(e1, ops)| {
        let mut res = e1;
        for (c, e2) in ops.into_iter() {
            match c.as_str() {
                "==" => res = Expr::Eq(Box::new(res), Box::new(e2)),
                "!=" => res = Expr::Neq(Box::new(res), Box::new(e2)),
                _ => unreachable!(),
            }
        }
        res
    });
    let eq = Rc::new(alt((eq, add_sub.clone())));

    alt((eq, add_sub, mul_div, deref, primary))
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_basics() {
        let s = "5678";
        let res = decimal().parse(s);
        assert_eq!(res, Ok(5678));

        let s = "0x1a3f";
        let res = hex_decimal().parse(s);
        assert_eq!(res, Ok(0x1a3f));

        let s = "$eax";
        let res = reg_name().parse(s);
        assert_eq!(res, Ok("eax".to_string()));
    }

    #[test]
    fn test_expr() {
        let s = "5678";
        let res = parser_expr().parse(s);
        assert_eq!(res, Ok(Expr::Decimal(5678)));
        let s = "0x1a3f";
        let res = parser_expr().parse(s);
        assert_eq!(res, Ok(Expr::Hexadecimal(0x1a3f)));
        let s = "$eax";
        let res = parser_expr().parse(s);
        assert_eq!(res, Ok(Expr::RegName("eax".to_string())));

        let res = parser_expr().parse("(123)");
        assert_eq!(res, Ok(Expr::Paren(Box::new(Expr::Decimal(123)))));

        let res = parser_expr().parse("(((123)))");
        assert_eq!(
            res,
            Ok(Expr::Paren(Box::new(Expr::Paren(Box::new(Expr::Paren(
                Box::new(Expr::Decimal(123))
            ))))))
        );

        let res = parser_expr().parse("1*2*3*4");
        assert_eq!(
            format!("{:?}", res),
            "Ok(Mul(Mul(Mul(Decimal(1), Decimal(2)), Decimal(3)), Decimal(4)))"
        );
        assert_eq!(res.unwrap().eval(&()), Ok(24));

        let res = parser_expr().parse("1/2");
        assert_eq!(format!("{:?}", res), "Ok(Div(Decimal(1), Decimal(2)))");

        let res = parser_expr().parse("1*2+2*3+3*4");
        assert_eq!(
            format!("{:?}", res),
            "Ok(Add(Add(Mul(Decimal(1), Decimal(2)), Mul(Decimal(2), Decimal(3))), Mul(Decimal(3), Decimal(4))))"
        );
        assert_eq!(res.unwrap().eval(&()), Ok(20));

        let res = parser_expr().parse("1-1*2");
        assert_eq!(
            format!("{:?}", res),
            "Ok(Sub(Decimal(1), Mul(Decimal(1), Decimal(2))))"
        );

        let res = parser_expr().parse("1==2*3-(4+5)+$eax");
        assert_eq!(
            format!("{:?}", res),
            "Ok(Eq(Decimal(1), Add(Sub(Mul(Decimal(2), Decimal(3)), Paren(Add(Decimal(4), Decimal(5)))), RegName(\"eax\"))))"
        );
        assert_eq!(
            res.unwrap().eval(&()),
            Err(EvalError::UnknownRegister("eax".to_string()))
        );

        let res = parser_expr().parse("**$eax");
        assert_eq!(format!("{:?}", res), "Ok(Deref(Deref(RegName(\"eax\"))))");
    }

    struct Regs(HashMap<String, i64>);
    impl Context for Regs {
        fn get_reg_value(&self, reg_name: &str) -> Option<i64> {
            self.0.get(reg_name).cloned()
        }

        fn get_pointer_value(&self, _addr: i64) -> Option<i64> {
            None // No pointer values in this test
        }
    }

    struct Deref;
    impl Context for Deref {
        fn get_reg_value(&self, _reg_name: &str) -> Option<i64> {
            None // No register values in this test
        }

        fn get_pointer_value(&self, addr: i64) -> Option<i64> {
            Some(addr + 1)
        }
    }

    #[test]
    fn test_complex() {
        let regs = Regs([("eax".to_string(), 10)].iter().cloned().collect());
        let res = parser_expr().parse("$eax==$eax");
        assert_eq!(
            format!("{:?}", res),
            "Ok(Eq(RegName(\"eax\"), RegName(\"eax\")))"
        );
        assert_eq!(res.unwrap().eval(&regs), Ok(true as i64));

        // might slow down the parsing if there exists too many brackets,
        // since too many backtraces
        let res = parser_expr().parse("1-(2-(3-(4-(5-6))))");
        assert_eq!(res.unwrap().eval(&()), Ok(-3));

        let res = parser_expr().parse("$eax==0x0000000a+1-1-(1-2)-1");
        assert_eq!(res.unwrap().eval(&regs), Ok(true as i64));

        let res = parser_expr().parse("***10");
        assert_eq!(res.unwrap().eval(&Deref), Ok(13));
    }
}
