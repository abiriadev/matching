use std::process::id;

enum Message2 {
    Hello { id: i32 },
}

enum Abiria {
    A(i32),
    B(i32),
    C(i32),
}

enum Hello {
    A(i32),
    B(i32),
    Struct { x: i32, y: i32 },
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8];

    while let Some(top) = v.pop() {
        println!("{}", top);
    }

    let tup = (3, 5);

    tuple(&tup);

    let s = Some(5);

    if let Some(_) = s {}

    if let x = 5 {
        println!("{}", x);
    }

    let x = 1;

    match x {
        1 => {}
        2 | 3 => (),
        4...20 => {}
        _ => (),
    }

    let ch = 'x';

    let is_alphabet = match ch {
        'a'...'z' | 'A'...'Z' => true,
        _ => false,
    };

    let Point { x, y } = Point { x: 10, y: 20 };

    println!("{}", x);
    println!("{}", y);

    let s = Point { x: 10, y: 20 };
    match s {
        Point { x, y: 0 } => x,
        Point { x: _, y } => y,
        _ => 0,
    };

    let m = Message::Move { x: 5, y: 7 };

    match m {
        Message::Quit => 0,
        Message::Move { x: 0, y: a } => a,
        Message::Move { x: _, y: a } => a * 2,
        Message::Write(str) => 0,
        Message::Color(_, _, b) => b,
    };

    let (_, _, first, .., last, _) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    println!("{}", first);
    println!("{}", last);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("{} is less than 5", x),
        Some(x) if x == 5 => println!("{} is 5", x),
        Some(x) => println!("{} is grater than 5", x),
        None => (),
    };

    // let opt_v = Some(5);
    //
    // if let Some(x) if x == 5 = opt_v {
    //
    // }

    let abiria = Abiria::A(10000);

    match abiria {
        Abiria::A(x) | Abiria::B(x) if x > 10 => println!("lorem"),
        Abiria::A(_) => {}
        Abiria::B(_) => {}
        Abiria::C(_) => {}
    };

    let some = Hello::Struct { x: 5, y: 6 };

    // if Hello::A(x) | Hello::Struct {x, ..} = some {
    // println!("Hello, {}", x);
    // }

    match some {
        Hello::A(x) | Hello::Struct { x, .. } if x > 4 => {}
        Hello::A(_) => {}
        Hello::B(_) => {}
        Hello::Struct { .. } => {}
    }

    let mm = Message2::Hello { id: 5 };

    match mm {
        Message2::Hello { id: id_var @ 3...7 } => println!("found id in the range: {}", id_var),
        Message2::Hello { id: 10...12 } => println!("found id in another range"),
        Message2::Hello { id } => println!("found other id: {}", id),
    };
}

fn tuple(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}
