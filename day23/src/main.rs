use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug)]
struct Expression {
    instruction: Operation,
    op_1: i64,
    op_2: String,
}

fn main() {

    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| it.len() > 0)
        .collect();

    let mut exprs = Vec::new();
    for line in lines {
        exprs.push(line_to_expression(&line));
    }

    let exprs = exprs;
    compute(&exprs);
    p2();
}

fn compute(exprs: &Vec<Expression>) {
    let mut mul_called = 0i64;
    let mut registers: HashMap<i64, i64> = HashMap::new();
    // set 'a' to 1
    registers.insert(0, 0);

    let mut index = 0i64;
    while index >= 0 && index < exprs.len() as i64 {
        let expr = &exprs[index as usize];
        let op = op_to_value(&mut registers, &expr.op_2);
        let register = registers.entry(expr.op_1).or_insert(0);

        // println!(
        //     "Action={:?} Register={}/{} Value={} Expr={:?}",
        //     expr.instruction,
        //     (expr.op_1 as u8 + 'a' as u8) as char,
        //     register,
        //     op,
        //     expr
        // );
        match expr.instruction {
            Operation::Set => *register = op,
            Operation::Sub => *register -= op,
            Operation::Mul => {
                *register *= op;
                mul_called += 1;
            }
            Operation::Jnz => {
                // Special case: Jnz op1 can be either a VALUE or a REGISTER; if negative,
                // it is a value.
                if expr.op_1 < 0 {
                    if (expr.op_1 + 'a' as i64) != 0 {
                        index += op - 1;
                    }
                } else if *register != 0 {
                    index += op - 1;
                }
            }
        }
        index += 1;
    }

    println!("P1: MUL called {} times", mul_called);
}

fn op_to_value(registers: &mut HashMap<i64, i64>, op: &str) -> i64 {
    let res = match op.parse::<i64>() {
        Ok(n) => n,
        Err(_) => {
            *registers
                .entry(op.as_bytes()[0] as i64 - 'a' as i64)
                .or_insert(0)
        }
    };
    res
}

fn line_to_expression(line: &str) -> Expression {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let instruction = match tokens[0].as_ref() {
        "set" => Operation::Set,
        "sub" => Operation::Sub,
        "mul" => Operation::Mul,
        "jnz" => Operation::Jnz,
        _ => {
            println!("KERNEL PANIC operation is not known");
            Operation::Set
        }
    };
    let op_1 = tokens[1].as_bytes()[0] as i64 - 'a' as i64;
    let op_2 = if tokens.len() >= 3 {
        tokens[2].to_string()
    } else {
        String::from("")
    };

    Expression {
        instruction: instruction,
        op_1: op_1,
        op_2: op_2,
    }
}


fn p2() {
    let C = 79*100 + 100000 + 17000;
    let mut b = 79*100 + 100000;
    let mut total = 0i64;
    while b <= C {
        let mut found_factors = false;
        for d in 2..b {
            for e in 2..b {
                // println!("d={} e={} b={} de-b={}", d, e, b, d*e-b);
                // If d,e product is b, then STOP the loops and increment the count,
                if d*e == b {
                    // println!("Match d,e,b={},{},{} de-b={}", d, e, b, d*e-b);
                    found_factors = true;
                    break;
                }
                // THIS IS THE SPEEDUP TRICK. Don't keep looking for a,b factors when the product is larger than the target.
                if d*e > b {
                    break;
                }
            }
            if found_factors {
                break;
            }
        }

        if found_factors {
            total += 1;
        }
        b += 17;
    }

    println!("P2: Value of h={}", total);
}
