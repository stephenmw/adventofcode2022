use std::collections::HashMap;

use arrayvec::ArrayString;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let vars = parse!(input);
    let root = vars.get("root").ok_or(anyhow!("root not found"))?;

    let ans = root.expand(&vars).simplify();
    let Expr::Value(ans) = ans else {bail!("expr did not fully simplify")};

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut vars = parse!(input);
    vars.remove("humn");

    let root = vars.get("root").ok_or(anyhow!("root not found"))?;
    let Expr::Operation(root_op) = root else {bail!("root is not an operation")};

    let a = root_op.a.expand(&vars).simplify();
    let b = root_op.b.expand(&vars).simplify();

    let (rhs, lhs) = match (a, b) {
        (rhs, Expr::Value(lhs)) => (rhs, lhs),
        (Expr::Value(lhs), rhs) => (rhs, lhs),
        _ => bail!("lhs or rhs must be a value"),
    };

    let ans = isolate_var(&rhs, lhs)?;

    Ok(ans.to_string())
}

fn isolate_var(mut rhs: &Expr, mut lhs: i64) -> anyhow::Result<i64> {
    loop {
        let op = match rhs {
            Expr::Operation(op) => op,
            Expr::Value(_) => bail!("no var in rhs"),
            Expr::Var(_) => return Ok(lhs),
        };

        let (e, val) = match (op.a.as_ref(), op.b.as_ref()) {
            (Expr::Value(v), b) => (b, v),
            (a, Expr::Value(v)) => (a, v),
            _ => bail!("rhs must have exactly one ident and be simplified"),
        };

        match op.op {
            Op::Add => {
                lhs -= val;
            }
            Op::Sub => {
                if op.a.is_value() {
                    lhs *= -1;
                }
                lhs += val;
            }
            Op::Mul => {
                lhs /= val;
            }
            Op::Div => {
                if !op.b.is_value() {
                    bail!("cannot divide by expression with ident");
                }
                lhs *= val;
            }
        }

        rhs = e;
    }
}

type Ident = ArrayString<4>;

#[derive(Clone, Debug)]
pub enum Expr {
    Operation(Operation),
    Value(i64),
    Var(Ident),
}

impl Expr {
    fn expand(&self, vars: &HashMap<Ident, Expr>) -> Expr {
        match self {
            Self::Operation(op) => {
                let new_op = Operation {
                    op: op.op,
                    a: op.a.expand(vars).into(),
                    b: op.b.expand(vars).into(),
                };
                Expr::Operation(new_op)
            }
            Self::Value(_) => self.clone(),
            Self::Var(var) => {
                let Some(e) = vars.get(var) else {return self.clone()};
                e.expand(vars)
            }
        }
    }

    fn simplify(&self) -> Expr {
        match self {
            Self::Operation(op) => {
                let a = op.a.simplify();
                let b = op.b.simplify();

                match (a, b) {
                    (Self::Value(a), Self::Value(b)) => Self::Value(op.op.execute(a, b).unwrap()),
                    (a, b) => Self::Operation(Operation {
                        op: op.op,
                        a: a.into(),
                        b: b.into(),
                    }),
                }
            }
            Self::Value(_) => self.clone(),
            Self::Var(_) => self.clone(),
        }
    }

    fn is_value(&self) -> bool {
        if let Self::Value(_) = self {
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operation(op) => write!(f, "({} {} {})", op.a, op.op.symbol(), op.b),
            Self::Value(x) => write!(f, "{}", x),
            Self::Var(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    op: Op,
    a: Box<Expr>,
    b: Box<Expr>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn execute(&self, a: i64, b: i64) -> Option<i64> {
        match self {
            Self::Add => a.checked_add(b),
            Self::Sub => a.checked_sub(b),
            Self::Mul => a.checked_mul(b),
            Self::Div => {
                if a.checked_rem(b)? == 0 {
                    a.checked_div(b)
                } else {
                    None
                }
            }
        }
    }

    fn symbol(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
        }
    }
}

mod parser {
    use std::collections::HashMap;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, HashMap<Ident, Expr>> {
        let parser = many1(ws_line(declaration)).map(|xs| xs.into_iter().collect());
        ws_all_consuming(parser)(input)
    }

    fn declaration(input: &str) -> IResult<&str, (Ident, Expr)> {
        let op = alt((
            value(Op::Add, char('+')),
            value(Op::Sub, char('-')),
            value(Op::Mul, char('*')),
            value(Op::Div, char('/')),
        ));
        let operation =
            tuple((ident, delimited(space0, op, space0), ident)).map(|(a, op, b)| Operation {
                op,
                a: Expr::Var(a).into(),
                b: Expr::Var(b).into(),
            });

        let expr = alt((
            uint.map(|x| Expr::Value(x)),
            operation.map(|x| Expr::Operation(x)),
        ));

        separated_pair(ident, tag(": "), expr).parse(input)
    }

    fn ident(input: &str) -> IResult<&str, Ident> {
        map_res(alpha1, ArrayString::from).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "152")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "301")
    }
}
