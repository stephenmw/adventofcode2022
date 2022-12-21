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
    let Expr::Operation(root_operation) = root else {bail!("root is not an operation")};

    let a = root_operation.a.expand(&vars).simplify();
    let b = root_operation.b.expand(&vars).simplify();

    println!("{} = {}", a, b);

    todo!()
}

type Ident = ArrayString<4>;

#[derive(Clone, Debug)]
pub enum Expr {
    Operation(Operation),
    Value(i64),
    Var(Ident),
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
        let parser = many1(ws_line(monkey)).map(|xs| xs.into_iter().collect());
        ws_all_consuming(parser)(input)
    }

    fn monkey(input: &str) -> IResult<&str, (Ident, Expr)> {
        let op = alt((
            value(Op::Add, char('+')),
            value(Op::Sub, char('-')),
            value(Op::Mul, char('*')),
            value(Op::Div, char('/')),
        ));
        let operation =
            tuple((var, delimited(space0, op, space0), var)).map(|(a, op, b)| Operation {
                op,
                a: Expr::Var(a).into(),
                b: Expr::Var(b).into(),
            });

        let expr = alt((
            uint.map(|x| Expr::Value(x)),
            operation.map(|x| Expr::Operation(x)),
        ));

        separated_pair(var, tag(": "), expr)
            .map(|(name, expr)| (name, expr))
            .parse(input)
    }

    fn var(input: &str) -> IResult<&str, Ident> {
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
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "301")
    }
}
