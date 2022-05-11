use core::panic;

use clap::{ArgEnum, Parser};
use vector::Vector;

#[derive(Debug, Clone, ArgEnum)]
enum OpType {
    Add,
    Sub,
    Mul,
    MulScalar,
    Div,
    Rem,
}

/// Calculates simple operations between vectors
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Operation to perform
    #[clap(short, long, arg_enum)]
    operation: OpType,

    /// Pretty print output (2x^3-x+5 instead of 2,0,-1,5)
    #[clap(short, long)]
    pretty_print: bool,

    /// First vector (format: 2,0,-1,5 = 2x^3-x+5)
    #[clap(parse(try_from_str=parse_vector))]
    op1: Vector<i32>,

    /// Second vector (or integer) (format: 2,0,-1,5 = 2x^3-x+5)
    #[clap(parse(try_from_str=parse_vector))]
    op2: Vector<i32>,
}

fn parse_vector(input: &str) -> Result<Vector<i32>, String> {
    let mut v: Vec<i32> = Vec::new();

    for n in input.split(",") {
        let n = n.trim().parse::<i32>().map_err(|e| e.to_string())?;
        v.push(n);
    }

    v.reverse();
    Ok(Vector::from(v))
}

fn fmt_vector(v: Vector<i32>) -> String {
    let mut v = v.as_slice();
    let mut str = String::new();

    for (i, val) in v.iter().enumerate().rev() {
        if i == 0 {
            str += &format!("{}", *val)
        } else {
            str += &format!("{},", *val)
        }
    }

    str
}

fn fmt_vector_pretty_print(v: Vector<i32>) -> String {
    let v = v.as_slice();
    let mut str = String::new();

    for (i, val) in v.iter().enumerate().rev() {
        if *val != 0 {
            if i == 0 {
                str += &format!(" {:+}", *val);
            } else {
                str += &format!(" {:+}x^{}", *val, i);
            }
        }
    }

    str
}

fn main() {
    let args = Args::parse();

    let result;

    match args.operation {
        OpType::Add => {
            result = args.op1 + args.op2;
        }
        OpType::Sub => {
            result = args.op1 - args.op2;
        }
        OpType::Div => {
            result = args.op1 / args.op2;
        }
        OpType::Rem => {
            result = args.op1 % args.op2;
        }
        OpType::Mul => {
            panic!("Mul not implemented!");
        }
        OpType::MulScalar => {
            let v2 = args.op2.as_slice();
            if v2.len() != 1 {
                panic!("Invalid scalar provided");
            }
            let n = args.op2.as_slice()[0];
            result = args.op1 * n;
        }
    }

    let result = if args.pretty_print {
        fmt_vector_pretty_print(result)
    } else {
        fmt_vector(result)
    };

    println!("{}", result)
}
