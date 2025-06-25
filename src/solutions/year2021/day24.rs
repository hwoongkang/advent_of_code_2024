use std::collections::HashSet;

// I CHEATED: https://github.com/VSZM/Advent_Of_Code/blob/master/2021/AOC2021/Day24.cs

use crate::Solution;

const _INPUT: &str = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -9
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y";

pub struct Day24 {}

#[derive(Copy, Clone)]
enum Value {
    Literal(i64),
    Memory(usize),
}

impl Value {
    fn from(s: &str) -> Self {
        match s.parse() {
            Ok(n) => Self::Literal(n),
            Err(_) => {
                let ind = s.chars().next().unwrap() as u32;
                let ind = ind - ('w' as u32);
                // w x y z
                let ind = (ind + 3) % 4;
                let ind = ind as usize;
                Self::Memory(ind)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Cmd {
    Inp(Value, i64),
    Mul(Value, Value),
    Add(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
    Eql(Value, Value),
}

impl Cmd {
    fn from(s: &str, buffer: &[i64], index: &mut usize) -> Self {
        let mut words = s.split_ascii_whitespace();
        match words.next().unwrap() {
            "inp" => {
                let n = buffer[*index];
                *index += 1;
                Self::Inp(Value::from(words.next().unwrap()), n)
            }
            "add" => Self::Add(
                Value::from(words.next().unwrap()),
                Value::from(words.next().unwrap()),
            ),
            "mul" => Self::Mul(
                Value::from(words.next().unwrap()),
                Value::from(words.next().unwrap()),
            ),
            "div" => Self::Div(
                Value::from(words.next().unwrap()),
                Value::from(words.next().unwrap()),
            ),
            "mod" => Self::Mod(
                Value::from(words.next().unwrap()),
                Value::from(words.next().unwrap()),
            ),
            "eql" => Self::Eql(
                Value::from(words.next().unwrap()),
                Value::from(words.next().unwrap()),
            ),
            _ => unreachable!(),
        }
    }
}

struct Alu {
    memory: [i64; 4],
}
impl Alu {
    fn new() -> Self {
        Self { memory: [0; 4] }
    }

    fn exec(&mut self, cmd: Cmd) {
        use Value::*;
        match cmd {
            Cmd::Inp(mem, val) => {
                let Memory(index) = mem else { panic!() };
                self.memory[index] = val;
            }
            Cmd::Add(x, y) => {
                let Memory(x) = x else { panic!() };
                let y = match y {
                    Memory(i) => self.memory[i],
                    Literal(i) => i,
                };
                self.memory[x] += y;
            }
            Cmd::Mul(x, y) => {
                let Memory(x) = x else { panic!() };
                let y = match y {
                    Memory(i) => self.memory[i],
                    Literal(i) => i,
                };
                self.memory[x] *= y;
            }
            Cmd::Div(x, y) => {
                let Memory(x) = x else { panic!() };
                let y = match y {
                    Memory(i) => self.memory[i],
                    Literal(i) => i,
                };
                self.memory[x] /= y;
            }
            Cmd::Mod(x, y) => {
                let Memory(x) = x else { panic!() };
                let y = match y {
                    Memory(i) => self.memory[i],
                    Literal(i) => i,
                };
                self.memory[x] %= y;
            }
            Cmd::Eql(x, y) => {
                let Memory(x) = x else { panic!() };
                let y = match y {
                    Memory(i) => self.memory[i],
                    Literal(i) => i,
                };
                if self.memory[x] == y {
                    self.memory[x] = 1;
                } else {
                    self.memory[x] = 0;
                }
            }
        }
    }
}

impl Solution for Day24 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        // reading..
        /*
        inp w // (0 0 0 n0)
        mul x 0 // (0 0 0 n0)
        add x z // (0 0 0 n0)
        mod x 26 // (0 0 0 n0)
        div z 1 // (0 0 0 n0)
        add x 12 // (12 0 0 n0)
        eql x w // (n0==12 0 0 n0)
        eql x 0 // (n0!=12 0 0 n0)
        mul y 0 // (n0!=12 0 0 n0)
        add y 25 // (n0!=12 25 0 n0)
        mul y x // (n0!=12 n0==12?0:25 0 n0)
        add y 1 // (n0!=12 n0==12?1:26 0 n0)
        mul z y // (n0!=12 n0==12?1:26 0 n0)
        mul y 0 // (n0!=12 0 0 n0)
        add y w // (n0!=12 n0 0 n0)
        add y 7 // (n0!=12 n0+7 0 n0)
        mul y x // (n0!=12 n0==12?0:n0+7 0 n0) but since n0 != 12
        add z y // (1 n0+7 n0+7 n0)
        inp w // (1 n0+7 n0+7 n1)
        mul x 0 // (0 n0+7 n0+7 n1)
        add x z // (n0+7 n0+7 n0+7 n1)
        mod x 26 // (n0+7 n0+7 n0+7 n1)
        div z 1 // (n0+7 n0+7 n0+7 n1)
        add x 11 // (n0+18 n0+7 n0+7 n1)
        eql x w // (0 n0+7 n0+7 n1)
        eql x 0 // (1 n0+7 n0+7 n1)
        mul y 0 // (1 0 n0+7 n1)
        add y 25 // (1 25 n0+7 n1)
        mul y x // (1 25 n0+7 n1)
        add y 1 // (1 26 n0+7 n1)
        mul z y // (1 26 26(n0+7) n1)
        mul y 0 // (1 0 26(n0+7) n1)
        add y w // (1 n1 26(n0+7) n1)
        add y 15 // (1 n1+15 26(n0+7) n1)
        mul y x // (1 n1+15 26(n0+7) n1)
        add z y // (1 n1+15 z n1) / z = 26(n0+7)+(n1+15)
        inp w // (1 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        mul x 0 // (0 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        add x z // (z n1+15 z n2) / z = 26(n0+7)+(n1+15)
        mod x 26 // (n1+15 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        div z 1 // (n1+15 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        add x 12 // (n1+27 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        eql x w // (0 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        eql x 0 // (1 n1+15 z n2) / z = 26(n0+7)+(n1+15)
        mul y 0 // (1 0 z n2) / z = 26(n0+7)+(n1+15)
        add y 25 // (1 25 z n2) / z = 26(n0+7)+(n1+15)
        mul y x // (1 25 z n2) / z = 26(n0+7)+(n1+15)
        add y 1 // (1 26 z n2) / z = 26(n0+7)+(n1+15)
        mul z y // (1 26 z n2) / z = 26**2(n0+7)+26(n1+15)
        mul y 0 // (1 0 z n2) / z = 26**2(n0+7)+26(n1+15)
        add y w // (1 n2 z n2) / z = 26**2(n0+7)+26(n1+15)
        add y 2 // (1 n2+2 z n2) / z = 26**2(n0+7)+26(n1+15)
        mul y x // (1 n2+2 z n2) / z = 26**2(n0+7)+26(n1+15)
        add z y // (1 n2+2 z2 n2) / z2 = 26**2(n0+7)+26(n1+15)+(n2+2)
        inp w // (1 n2+2 z n3) / z = 26**2(n0+7)+26(n1+15)+(n2+2)
        mul x 0 // (0 n2+2 z n3) / z = 26**2(n0+7)+26(n1+15)+(n2+2)
        add x z // (z n2+2 z n3) / z = 26**2(n0+7)+26(n1+15)+(n2+2)
        mod x 26 // (n2+2 n2+2 z n3) / z = 26**2(n0+7)+26(n1+15)+(n2+2)
        div z 26 // (n2+2 n2+2 z n3) / z = 26(n0+7)+(n1+15)
        add x -3 // (n2-1 n2+2 z n3) / z = 26(n0+7)+(n1+15)
        eql x w // (n3 == n2-1 n2+2 z n3) / z = 26(n0+7)+(n1+15)
        eql x 0 // (n3!=n2-1 n2+2 z n3) / z = 26(n0+7)+(n1+15)
        mul y 0 // (n3!=n2-1 0 z n3) / z = 26(n0+7)+(n1+15)
        add y 25 // (n3!=n2-1 25 z n3) / z = 26(n0+7)+(n1+15)
        mul y x // (n3!=n2-1 n3!=n2-1?25:0 z n3)
        add y 1 // (n3!=n2-1 n3!=n2-1?26:1 z n3)
        mul z y // (n3!=n2-1 n3!=n2-1?26:1 n3!=n2-1?26z:z n3) / z = 26(n0+7)+(n1+15)
        mul y 0 // (n3!=n2-1 0 n3!=n2-1?26z:z n3) / z = 26(n0+7)+(n1+15)
        add y w // (n3!=n2-1 n3 n3!=n2-1?26z:z n3) / z = 26(n0+7)+(n1+15)
        add y 15 // (n3!=n2-1 n3+15 n3!=n2-1?26z:z n3) / z = 26(n0+7)+(n1+15)
        mul y x // (n3!=n2-1 n3!=n2-1?n3+15:0 n3!=n2-1?26z:z n3) / z = 26(n0+7)+(n1+15)
        add z y // (x y z3 n3) / z3 = n3!=n2-1 ? 26**2(n0+7)+26(n1+15)+n3+15 : 26(n0+7)+(n1+15)
        inp w // (x y z n4)
        mul x 0 // (0 y z n4)
        add x z // (z y z n4)
        mod x 26 // (f(n2, n3) y z n4)
        div z 1 // (f(n2, n3) y z n4)
        add x 10 // (f(n2, n3)+10 y z n4)
        eql x w // (0 y z n4)
        eql x 0 // (1 y z n4)
        mul y 0 // (1 0 z n4)
        add y 25 // (1 25 z n4)
        mul y x // (1 25 z n4)
        add y 1 // (1 26 z n4)
        mul z y // (1 26 26*z n4)
        mul y 0 // (1 0 26z n4)
        add y w // (1 n4 26z n4)
        add y 14 // (1 n4+14 26z n4)
        mul y x // (1 n4+14 26z n4)
        add z y // (1 n4+14 z4 n4) / z4 = 26*z3 + (n4+14)
        inp w // (1 n4+14 z4 n5)
        mul x 0 // (0 n4+14 z4 n5)
        add x z // (z4 n4+14 z4 n5)
        mod x 26 // (n4+14 n4+14 z4 n5)
        div z 26 // (n4+14 n+14 z3 n5)
        add x -9 // (n4+5 n+14 z3 n5)
        eql x w // (n4+5==n5 n+14 z3 n5)
        eql x 0 // (n4+5!=n5 n+14 z3 n5)
        mul y 0 // (n4+5!=n5 0 z3 n5)
        add y 25 // (n4+5!=n5 25 z3 n5)
        mul y x //
        add y 1 // (n4+5!=n5 n4+5!=n5?26:1 z3 n5)
        mul z y // (n4+5!=n5 n4+5!=n5?26:1 z5 n5) / z5 = n4+5!=n5 ? 26*z3 : z3
        mul y 0 // (n4+5!=n5 0 z5 n5) / z5 := n4+5!=n5 ? 26*z3 : z3
        add y w // (n4+5!=n5 n5 z5 n5) / z5 := n4+5!=n5 ? 26*z3 : z3
        add y 2 // (n4+5!=n5 n5+2 z5 n5) / z5 := n4+5!=n5 ? 26*z3 : z3
        mul y x //
        add z y // (x5 y5 z5 n5) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        inp w // (x5 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul x 0 // (0 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add x z // (z5 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mod x 26 // (z5 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        div z 1 // (z5 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add x 10 // (z5 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        eql x w // (0 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        eql x 0 // (1 y5 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y 0 // (1 0 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y 25 // (1 25 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y x // (1 25 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y 1 // (1 26 z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul z y // (1 26 26*z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y 0 // (1 0 26*z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y w // (1 n6 26*z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y 15 // (1 n6+15 26*z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y x // (1 n6+15 26*z5 n6) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add z y // (1 n6+15 z6 n6) / z6:= 26 * z5 + (n6+15)
        inp w // (1 n6+15 z6 n7) / z6:= 26 * z5 + (n6+15)
        mul x 0 // (0 n6+15 z6 n7) / z6:= 26 * z5 + (n6+15)
        add x z // (z6 n6+15 z6 n7) / z6:= 26 * z5 + (n6+15)
        mod x 26 // (n6+15 n6+15 z6 n7) / z6:= 26 * z5 + (n6+15)
        div z 26 // (n6+15 n6+15 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add x -7 // (n6+8 n6+15 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        eql x w // (n6+8==n7 n6+15 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        eql x 0 // (n6+8!=n7 n6+15 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y 0 // (n6+8!=n7 0 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y 25 // (n6+8!=n7 25 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul y x // (n6+8!=n7 n6+8!=n7?25:0 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        add y 1 // (n6+8!=n7 n6+8!=n7?26:1 z5 n7) / z5 := n4+5!=n5 ? 26*z3 + (n5+2) : z3
        mul z y // (n6+8!=n7 n6+8!=n7?26:1 z7 n7) / z7 := n6+8!=n7 ? 26*z5 : z5
        mul y 0 // (n6+8!=n7 0 z7 n7) / z7 := n6+8!=n7 ? 26*z5 : z5
        add y w // (n6+8!=n7 n7 z7 n7) / z7 := n6+8!=n7 ? 26*z5 : z5
        add y 1 // (n6+8!=n7 n7+1 z7 n7) / z7 := n6+8!=n7 ? 26*z5 : z5
        mul y x // (n6+8!=n7 n7+1 z7 n7) / z7 := n6+8!=n7 ? 26*z5 : z5
        add z y // (n6+8!=n7 n7+1 z7 n7) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        inp w // (n6+8!=n7 n7+1 z7 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mul x 0 // (0 n7+1 z7 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        add x z // (z7 n7+1 z7 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mod x 26 // (z7%26 n7+1 z7/26 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        div z 26 // (z7%26 n7+1 z7/26 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        add x -11 // (z7%26-11 n7+1 z7/26 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        eql x w // (z7%26-11 n7+1 z7/26 n8) / z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        eql x 0 // (x8 n7+1 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mul y 0 // (x8 0 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        add y 25 // (x8 25 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mul y x // (x8 x8?25:0 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        add y 1 // (x8 x8?26:1 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mul z y // (x8 x8?25:0 z7/26 n8) / x8 := z7%26!=n8+11, z7 := n6+8!=n7 ? 26*z5 + (n7 + 1) : z5
        mul y 0
        add y w
        add y 15
        mul y x
        add z y // (x8 _ z8 n8) / z8 = z7%26 != n8 + 11 ? 26 * (z7 / 26) + (n8 + 15) : z7/26
        inp w // (x8 _ z8 n9)
        mul x 0 // (0 _ z8 n9)
        add x z // (z8 _ z8 n9)
        mod x 26 // (x9 _ n0+7 n9) / x9 = z8%26, z9 = z8/26
        div z 26 // (x9 _ z9 n9)
        add x -4 // (x9-4 _ z9 n9)
        eql x w // (x9-4==n9 _ z9 n9)
        eql x 0 // (x9-4!=n9 _ z9 n9) x9 = z8%26, z9 = z8/26
        mul y 0 // (x9-4!=n9 0 z9 n9) x9 = z8%26, z9 = z8/26
        add y 25 // (x9-4!=n9 25 z9 n9) x9 = z8%26, z9 = z8/26
        mul y x // (x9 x9?25:0 0 n9) x9 := n0+3!=n9
        add y 1 // (x9 x9?26:1 0 n9) x9 := n0+3!=n9
        mul z y // (x9 x9?26:1 0 n9) x9 := n0+3!=n9
        mul y 0
        add y w
        add y 15
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 14
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 12
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 11
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 2
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -8
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 13
        mul y x
        add z y
        inp w // (x12 y12 z12 n13)
        mul x 0 // (0 y12 z12 n13)
        add x z // (z12 y12 z12 n13)
        mod x 26 // (x13 y12 z12 n13) / x13 := z12 % 26
        div z 26 // (x13 y12 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        add x -10 // (x13-10 y12 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        eql x w // (x13-10==n13 y12 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        eql x 0 // (x13-10!=n13 y12 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        mul y 0 // (x13-10!=n13 0 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        add y 25 // (x13-10!=n13 25 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        mul y x // (x13-10!=n13 x13-10!=n13?25:0 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        add y 1 // (x13-10!=n13 x13-10!=n13?26:1 z13 n13) / x13 := z12 % 26, z13 := z12 / 26
        mul z y // (x13-10!=n13 y13 y13 * z13 n13) / x13 := z12 % 26, y13 := x13-10!=n13?26:1, z13 := z12 / 26
        mul y 0 // (x13-10!=n13 0 y13 * z13 n13) / x13 := z12 % 26, y13 := x13-10!=n13?26:1, z13 := z12 / 26
        add y w
        add y 13 // (x13-10!=n13 n13+13 y13 * z13 n13) / x13 := z12 % 26, y13 := x13-10!=n13?26:1, z13 := z12 / 26
        mul y x // x must be 0
        add z y / z13 must be 0: y must be 0
         */
        let dummy = [0; 14];
        let mut index = 0;
        let cmds: Vec<_> = input
            .lines()
            .map(|line| Cmd::from(line, &dummy, &mut index))
            .collect();

        fn backtrack(
            depth: usize,
            model_number: i64,
            prev_z: i64,
            cmds: &[Cmd],
            cache: &mut HashSet<(usize, i64)>,
        ) -> Option<i64> {
            if depth == 14 || cache.contains(&(depth, prev_z)) {
                return None;
            }
            let model_number = model_number * 10;
            for i in (1..=9).rev() {
                let mut alu = Alu::new();
                let start = 18 * depth;
                alu.memory = [0, 0, prev_z, 0];
                for cmd in &cmds[start..start + 18] {
                    let cmd = match cmd {
                        Cmd::Inp(val, _) => Cmd::Inp(*val, i),
                        cmd => *cmd,
                    };
                    alu.exec(cmd);
                }
                let z = alu.memory[2];
                if alu.memory[2] == 0 && depth == 13 {
                    return Some(model_number + i);
                }
                let next = backtrack(depth + 1, model_number + i, z, cmds, cache);
                if next.is_some() {
                    return next;
                }
            }
            cache.insert((depth, prev_z));
            None
        }
        let mut cache = HashSet::new();
        backtrack(0, 0, 0, &cmds, &mut cache).unwrap().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let dummy = [0; 14];
        let mut index = 0;
        let cmds: Vec<_> = input
            .lines()
            .map(|line| Cmd::from(line, &dummy, &mut index))
            .collect();

        fn backtrack(
            depth: usize,
            model_number: i64,
            prev_z: i64,
            cmds: &[Cmd],
            cache: &mut HashSet<(usize, i64)>,
        ) -> Option<i64> {
            if depth == 14 || cache.contains(&(depth, prev_z)) {
                return None;
            }
            let model_number = model_number * 10;
            for i in 1..=9 {
                let mut alu = Alu::new();
                let start = 18 * depth;
                alu.memory = [0, 0, prev_z, 0];
                for cmd in &cmds[start..start + 18] {
                    let cmd = match cmd {
                        Cmd::Inp(val, _) => Cmd::Inp(*val, i),
                        cmd => *cmd,
                    };
                    alu.exec(cmd);
                }
                let z = alu.memory[2];
                if alu.memory[2] == 0 && depth == 13 {
                    return Some(model_number + i);
                }
                let next = backtrack(depth + 1, model_number + i, z, cmds, cache);
                if next.is_some() {
                    return next;
                }
            }
            cache.insert((depth, prev_z));
            None
        }
        let mut cache = HashSet::new();
        backtrack(0, 0, 0, &cmds, &mut cache).unwrap().to_string()
    }
}

#[cfg(test)]
mod day24_tests {
    use rand::{rng, Rng};

    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_2(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_first_digit() {
        let input = _INPUT.to_string();
        for num in 1..=12 {
            let nums = [num];
            let mut index = 0;
            let mut alu = Alu::new();
            for line in input.lines().take(18) {
                let cmd = Cmd::from(line, &nums, &mut index);
                alu.exec(cmd);
            }
            let memory = if num == 12 {
                [0, 0, 0, 12]
            } else {
                [1, num + 7, num + 7, num]
            };
            assert_eq!(memory, alu.memory)
        }
    }

    #[test]
    fn digits() {
        let input = _INPUT.to_string();
        let trials = 50_000;

        for _ in 0..trials {
            let nums: Vec<i64> = (0..14).map(|_| rng().random_range(1..=9)).collect();
            let mut index = 0;
            let mut alu = Alu::new();
            let n0 = nums[0];
            let n1 = nums[1];
            let n2 = nums[2];
            let n3 = nums[3];
            let n4 = nums[4];
            let n5 = nums[5];
            let n6 = nums[6];
            let n7 = nums[7];
            let n8 = nums[8];

            let z0 = n0 + 7;
            let z1 = 26 * z0 + (n1 + 15);
            let z2 = 26 * z1 + (n2 + 2);
            let z3 = if n3 + 1 != n2 {
                26 * z1 + (n3 + 15)
            } else {
                z1
            };
            let z4 = 26 * z3 + (n4 + 14);
            let z5 = if n4 + 5 != n5 {
                //
                26 * z3 + (n5 + 2)
            } else {
                z3
            };
            let z6 = 26 * z5 + (n6 + 15);
            let z7 = if n6 + 8 != n7 {
                /* */
                26 * z5 + (n7 + 1)
            } else {
                z5
            };

            let z8 = if z7 % 26 != n8 + 11 {
                26 * (z7 / 26) + (n8 + 15)
            } else {
                z7 / 26
            };

            let zs = [z0, z1, z2, z3, z4, z5, z6, z7, z8];

            for (count, line) in input.lines().enumerate() {
                let cmd = Cmd::from(line, &nums, &mut index);
                alu.exec(cmd);
                if count % 18 == 17 {
                    let cycle = count / 18;
                    if cycle >= zs.len() {
                        break;
                    }
                    assert_eq!(alu.memory[2], zs[cycle])
                }
            }
        }
    }

    #[test]
    fn test_so_far() {
        // sofar

        // if n6 + 8 == n7
        //  then z7 = z5
        // if n4 + 5 == n5
        //  then z7 = z5 = z3
        // if n3 + 1 == n2
        //  then z7 = z5 = z3 = z1
        // which means
        // if n1 + 15 == n8 + 11
        //  then z8 = z0 = n0+7
        // thus
        // n1 == n8 - 4
        // n3 == n2 - 1
        // n4 == n5 - 5
        // n6 == n7 - 8

        let input = Day24::test_input();
        for n7 in 9..=9 {
            for n5 in 6..=9 {
                for n2 in 2..=9 {
                    for n8 in 5..=9 {
                        for n0 in 1..=9 {
                            let nums = [n0, n8 - 4, n2, n2 - 1, n5 - 5, n5, n7 - 8, n7, n8];
                            let mut alu = Alu::new();
                            let mut index = 0;
                            for (count, line) in input.lines().enumerate() {
                                let cmd = Cmd::from(line, &nums, &mut index);
                                alu.exec(cmd);
                                if count % 18 == 17 {
                                    let cycle = count / 18;
                                    if cycle == nums.len() - 1 {
                                        assert_eq!(alu.memory[2], n0 + 7);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
