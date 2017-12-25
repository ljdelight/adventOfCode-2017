use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
#[derive(Clone)]
struct Expression {
    instruction: String,
    op_1: String,
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
    let (a_tx, a_rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (b_tx, b_rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let inputA = exprs.clone();
    let a = thread::spawn(move || { compute(&inputA, 0, &b_tx, &a_rx); });

    let inputB = exprs.clone();
    let b = thread::spawn(move || compute(&inputB, 1, &a_tx, &b_rx));

    let resA = a.join().unwrap();
    let resB = b.join().unwrap();
    // println!("Last sounds was {} with {} sends", resA.0, resA.1);

}

fn compute(
    exprs: &Vec<Expression>,
    id: i64,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) -> (i64, i64) {
    let mut last_sound = 0;
    let mut values_sent = 0;
    let mut registers: HashMap<String, i64> = HashMap::new();
    registers.insert(String::from("p"), id);
    let mut index = 0i64;
    while index >= 0 && index < exprs.len() as i64 {
        // println!("{}: Inspecting idx {}", id, index);
        let expr = &exprs[index as usize];
        let register = expr.op_1.to_string();
        match expr.instruction.as_ref() {
            "snd" => {
                last_sound = *registers.entry(register).or_insert(0);
                sender.send(last_sound);
                values_sent += 1;
                println!("{}: Sending {}, ttl {}", id, last_sound, values_sent);
            }
            "rcv" => {
                let recv = receiver.recv().unwrap();
                *registers.entry(register).or_insert(0) = recv;
                // println!("{}: Received {}", id, recv);
                // if *registers.entry(register).or_insert(0) != 0 {
                //     println!("Rcv last sound {}", last_sound);
                //     index = -11;
                //     break;
                // }
                // println!("RCV Skipping receive (we're 0)");
            }
            "set" => {
                // println!("{}: SET Register {} set to {}", id, register, expr.op_2);
                let op = op_to_value(&mut registers, &expr.op_2);
                *registers.entry(register).or_insert(0) = op;
            }
            "add" => {
                // println!("{}: ADD Register {} add {}", id, register, expr.op_2);
                let op = op_to_value(&mut registers, &expr.op_2);
                *registers.entry(register).or_insert(0) += op;
            }
            "mul" => {
                // println!("{}: MUL Register {} mul {}", id, register, expr.op_2);
                let op = op_to_value(&mut registers, &expr.op_2);
                *registers.entry(register).or_insert(0) *= op;
            }
            "mod" => {
                // println!("{}: MOD Register {} mod {}", id, register, expr.op_2);
                let op = op_to_value(&mut registers, &expr.op_2);
                *registers.entry(register).or_insert(0) %= op;
            }
            "jgz" => {
                if *registers.entry(register).or_insert(0) > 0 {
                    // println!("{}: JGZ by {}", id, expr.op_2);
                    let op = op_to_value(&mut registers, &expr.op_2);
                    index += op - 1;
                } else {
                    // println!("{}: JGZ Not jumping (we're 0)", id);
                }
            }
            _ => {
                // println!("{}: BAD INPUT", id);
            }
        }
        index += 1;
    }
    (last_sound, values_sent)
}

fn op_to_value(registers: &mut HashMap<String, i64>, op: &str) -> i64 {
    let mut res = match op.parse::<i64>() {
        Ok(n) => n,
        Err(_) => *registers.entry(op.to_string()).or_insert(0),
    };
    // println!("Found {} -> {}", op, res);
    res
}

fn line_to_expression(line: &str) -> Expression {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let instruction = tokens[0].to_string();
    let op_1 = tokens[1].to_string();
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
