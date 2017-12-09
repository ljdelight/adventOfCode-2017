use std::io;
use std::io::BufRead;
use std::collections::HashMap;

struct Expression {
    variable: String,
    op: String,
    value: i64,
}

impl Expression {
    fn evaluate(&self, vars: &mut HashMap<String, i64>) -> bool {
        // TODO: copying the string shouldn't be necessary. HTF do we use 'variable',
        // which doesn't go out of scope, as the key???
        let val = vars.entry(self.variable.to_string()).or_insert(0);
        return match self.op.as_ref() {
            "inc" => {
                *val += self.value;
                true
            }
            "dec" => {
                *val -= self.value;
                true
            }
            "<" => *val < self.value,
            "<=" => *val <= self.value,
            "==" => *val == self.value,
            ">=" => *val >= self.value,
            ">" => *val > self.value,
            "!=" => *val != self.value,
            _ => {
                println!("Failed to evaluate");
                false
            }
        };
    }
}

fn to_expression(part: &str) -> Expression {
    let tokens: Vec<&str> = part.split_whitespace().collect();
    Expression {
        variable: String::from(tokens[0]),
        op: String::from(tokens[1]),
        value: tokens[2].parse().unwrap(),
    }
}

fn parse_line(line: &String) -> (Expression, Expression) {
    let split: Vec<&str> = line.split("if").collect();
    let t = to_expression(&split[0]);
    let u = to_expression(&split[1]);
    (t, u)
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|res| res.unwrap())
        .collect();

    let mut globalMaxVal = -1i64;
    let mut globalMaxVar: String = String::from("");
    let mut varMap = HashMap::new();
    for line in lines {
        let exprs = parse_line(&line);
        if exprs.1.evaluate(&mut varMap) {
            exprs.0.evaluate(&mut varMap);

            // Update the global max
            for (var, &val) in varMap.iter() {
                if val > globalMaxVal {
                    globalMaxVal = val;
                    globalMaxVar = var.clone();
                }
            }
        }
    }

    let mut maxVal = -1i64;
    let mut maxVar = "";
    for (var, &val) in varMap.iter() {
        if val > maxVal {
            maxVal = val;
            maxVar = var;
        }
    }

    println!(
        "Global maximum register is {}={}",
        globalMaxVar,
        globalMaxVal
    );
    println!("End of process max is {}={}", maxVar, maxVal);
}
