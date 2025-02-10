fn takes_ownership(some_string:String){
    println!("{ }", some_string);
}

fn makes_copy(f:i32){
    println!("{ }", f);
}

struct Person{
    name: String,
    age: u8,
}

enum Direction{
    Up,
    Down,
    Right,
    Left,
}

fn move_direction(dir: Direction){
    match dir{
        Direction::Up => println!("Moving up"),
        _=> println!("Other Direction"),
    }
}


fn main() {
    let x=5;
    let mut y=10;
    let mut i=0;
    let g: String=String::from("Rust");
    let s: String=String::from("Hello");
    y+=5;
    println!("x:{ }, y:{ }",x,y);
    println!("String: { }", g);
    takes_ownership(s);

    let p=Person{
        name: String::from("Sony"),
        age: 25,
    };

    println!("{} is { } years old", p.name, p.age);

    let direction=Direction::Up;
    move_direction(direction);

    let z=33;
    makes_copy(z);

    if x>y {
        println!("X is greater");
    }else{
        println!("Y is greater");
    }

    for _ in 0..3{
        println!("Printing num:{ }", i);
        i+=1;
    }
}
