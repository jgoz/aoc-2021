use std::{env, io, io::prelude::*, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).cloned().unwrap_or(String::from("1"));

    let reader = io::stdin();
    let lines = reader.lock().lines();
    let values = lines.map(|x| x.unwrap());

    match part.as_str() {
        "1" => println!("{}", day18_part1(values)),
        "2" => println!("{}", day18_part2(values)),
        _ => println!("Invalid part {}", part),
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Open,
    Close,
    Value(i32),
}

type Tokens = Vec<Token>;

fn tokenize(text: &str) -> Tokens {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for c in text.chars() {
        if buffer.len() > 0 && (c == ']' || c == ',') {
            match buffer.parse::<i32>() {
                Ok(number) => {
                    tokens.push(Token::Value(number));
                    buffer = String::new();
                }
                Err(msg) => panic!("Failed to parse number: {}", msg),
            }
        }
        match c {
            ' ' => (),
            ',' => (),
            '[' => tokens.push(Token::Open),
            ']' => tokens.push(Token::Close),
            '0'..='9' => buffer.push(c),
            _ => panic!("Unexpected character: '{}'", c),
        }
    }

    tokens
}

pub fn add_comma(prev: Option<&Token>, result: &mut String) {
    match prev {
        Some(&Token::Value(_)) => (*result).push(','),
        Some(&Token::Close) => (*result).push(','),
        _ => (),
    }
}

pub fn stringify(tokens: &Tokens) -> String {
    let mut result = String::new();
    let mut prev: Option<&Token> = None;
    for token in tokens {
        match token {
            Token::Open => {
                add_comma(prev, &mut result);
                result.push('[')
            }
            Token::Close => result.push(']'),
            Token::Value(number) => {
                add_comma(prev, &mut result);
                result.push_str(&number.to_string())
            }
        }
        prev = Some(token)
    }
    result
}

fn explode(tokens: &Tokens) -> Option<Tokens> {
    let mut depth = 0;
    let mut exploded = vec![];
    let mut iter = tokens.into_iter();
    let mut did_explode = false;

    while let Some(token) = iter.next() {
        match token {
            Token::Open => {
                depth += 1;
                exploded.push(token.clone());
            }
            Token::Close => {
                depth -= 1;
                exploded.push(token.clone());
            }
            Token::Value(val_left) => {
                if depth != 5 || did_explode {
                    exploded.push(token.clone());
                    continue;
                }

                // At depth > 4, explode
                let val_right = match iter.next().unwrap() {
                    Token::Value(val) => val,
                    _ => panic!("Expected value"),
                };

                // Add left value to previous value, if any
                let mut restore = vec![];
                while let Some(left) = exploded.pop() {
                    match left {
                        Token::Value(v) => {
                            restore.push(Token::Value(v + val_left));
                            break;
                        }
                        tok => restore.push(tok),
                    }
                }
                for tok in restore.into_iter().rev() {
                    exploded.push(tok);
                }

                // Replace empty pair with 0
                exploded.pop();
                exploded.push(Token::Value(0));
                iter.next();

                // Add value to next value, if any
                while let Some(right) = iter.next() {
                    match right {
                        Token::Value(v) => {
                            exploded.push(Token::Value(v + val_right));
                            break;
                        }
                        tok => exploded.push(tok.clone()),
                    }
                }
                did_explode = true;
            }
        }
    }

    if did_explode {
        Some(exploded)
    } else {
        None
    }
}

#[test]
fn explode_test() {
    // null case
    assert!(explode(&tokenize("[[[0,9],2],3]")).is_none());
    assert_eq!(
        "[[[[0,9],2],3],4]",
        stringify(&explode(&tokenize("[[[[[9,8],1],2],3],4]")).unwrap())
    );
    assert_eq!(
        "[7,[6,[5,[7,0]]]]",
        stringify(&explode(&tokenize("[7,[6,[5,[4,[3,2]]]]]")).unwrap())
    );
    assert_eq!(
        "[[6,[5,[7,0]]],3]",
        stringify(&explode(&tokenize("[[6,[5,[4,[3,2]]]],1]")).unwrap())
    );
    assert_eq!(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        stringify(&explode(&tokenize("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")).unwrap())
    );
    assert_eq!(
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        stringify(&explode(&tokenize("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")).unwrap())
    );
    assert_eq!(
        "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        stringify(&explode(&tokenize("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")).unwrap())
    );
}

fn split(tokens: &Tokens) -> Option<Tokens> {
    let mut split = vec![];
    let mut iter = tokens.into_iter();
    let mut did_split = false;

    while let Some(token) = iter.next() {
        match token {
            Token::Open => {
                split.push(token.clone());
            }
            Token::Close => {
                split.push(token.clone());
            }
            Token::Value(value) => {
                if *value >= 10 && !did_split {
                    split.push(Token::Open);
                    split.push(Token::Value(value / 2));
                    split.push(Token::Value((value / 2) + (value % 2)));
                    split.push(Token::Close);
                    did_split = true;
                } else {
                    split.push(token.clone());
                }
            }
        }
    }

    if did_split {
        Some(split)
    } else {
        None
    }
}

#[test]
fn split_tests() {
    assert_eq!(
        "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        stringify(&split(&tokenize("[[[[0,7],4],[15,[0,13]]],[1,1]]")).unwrap())
    );
    assert_eq!(
        "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        stringify(&split(&tokenize("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")).unwrap())
    )
}

fn add(left: Tokens, right: Tokens) -> Tokens {
    let mut cur = vec![Token::Open];
    cur.append(&mut left.clone());
    cur.append(&mut right.clone());
    cur.push(Token::Close);

    loop {
        if let Some(exploded) = explode(&cur) {
            cur = exploded;
        } else if let Some(split) = split(&cur) {
            cur = split;
        } else {
            break;
        }
    }
    cur
}

#[test]
fn add_tests() {
    assert_eq!(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        stringify(&add(
            tokenize("[[[[4,3],4],4],[7,[[8,4],9]]]"),
            tokenize("[1,1]"),
        ))
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Val(i32),
    Pair(Box<Node>, Box<Node>),
}

impl Node {
    fn from(tokens: impl Iterator<Item = Token>) -> Node {
        let mut stack = vec![];

        for next in tokens {
            match next {
                Token::Open => {
                    stack.push(Node::Pair(Box::new(Node::Val(0)), Box::new(Node::Val(0))));
                }
                Token::Value(value) => {
                    stack.push(Node::Val(value));
                }
                Token::Close => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.pop(); // Pair

                    stack.push(Node::Pair(Box::new(left), Box::new(right)));
                }
            }
        }

        stack.pop().unwrap()
    }

    fn magnitude(&self) -> i32 {
        match self {
            Node::Val(v) => *v,
            Node::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

#[macro_export]
macro_rules! fishy {
    // Terminating case
    () => (());

    // Literal value
    ($v:literal) => (Node::Val($v));

    // Initial case (allows us to use [] as start/end tokens for the macro)
    ($($left:tt)?, $($right:tt)?) => {{
        Node::Pair(Box::new(fishy!($($left)?)), Box::new(fishy!($($right)?)))
    }};

    // Unpack an array into Pair expressions
    ([ $($left:tt)?, $($right:tt)? ]) => {{
        Node::Pair(Box::new(fishy!($($left)?)), Box::new(fishy!($($right)?)))
    }};
}

#[test]
fn node_from_test() {
    assert_eq!(
        fishy![[1, 2], 3],
        Node::from(tokenize("[[1,2],3]").into_iter()),
    );

    assert_eq!(
        fishy![[1, 2], [[3, 4], 5]],
        Node::from(tokenize("[[1,2],[[3,4],5]]").into_iter()),
    );
}

fn day18_part1(mut v: impl Iterator<Item = String>) -> i32 {
    let first = v.next().unwrap();
    let mut left = tokenize(&first);
    while let Some(cur) = v.next() {
        let right = tokenize(&cur);
        left = add(left, right);
    }

    let node = Node::from(left.into_iter());
    node.magnitude()
}

#[test]
fn day18_part1_test() {
    let v = vec![
        String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
        String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
        String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
        String::from("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
        String::from("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
        String::from("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
        String::from("[[[[5,4],[7,7]],8],[[8,3],8]]"),
        String::from("[[9,3],[[9,9],[6,[4,9]]]]"),
        String::from("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
        String::from("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
    ];
    let answer = day18_part1(v.into_iter());

    assert_eq!(4140, answer);
}

fn day18_part2(v: impl Iterator<Item = String>) -> i32 {
    let mut max = 0;

    let lines = v.collect::<Vec<_>>();

    for left in lines.iter() {
        for right in lines.iter() {
            if left == right {
                continue;
            }
            let sum = add(tokenize(left), tokenize(right));
            let node = Node::from(sum.into_iter());
            let magnitude = node.magnitude();

            if magnitude > max {
                max = magnitude;
            }
        }
    }

    max
}

#[test]
fn day18_part2_test() {
    let v = vec![
        String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
        String::from("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
        String::from("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
        String::from("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
        String::from("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
        String::from("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
        String::from("[[[[5,4],[7,7]],8],[[8,3],8]]"),
        String::from("[[9,3],[[9,9],[6,[4,9]]]]"),
        String::from("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
        String::from("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
    ];
    let answer = day18_part2(v.into_iter());

    assert_eq!(3993, answer);
}
