/**
 * The keyword "loop" alone will execute the code block inside in an infinite
 * loop. The "break" keyword will cause the loop to exit. The "continue"
 * keyword will skip all remaining code in the code block and start the next
 * loop immediately
 */
fn fibloop(n: i32) {  // print the first n fibonacci numbers
    let mut p1: i32 = 0;
    let mut p2: i32 = 1;
    let mut num: i32 = p1 + p2;
    let mut count: i32 = 0;
    
    loop {
        if count >= n { break; }
        if count == 0 {
            println!("{p1}");
        } else if count == 1 {
            println!("{p2}");
        } else {
            println!("{num}");
            p1 = p2;
            p2 = num;
            num = p1 + p2;
        }
        count += 1;
    }
}

fn fibwhile(n: i32) { // same as fibloop but using a while loop
    let mut p1: i32 = 0;
    let mut p2: i32 = 1;
    let mut num: i32 = p1 + p2;
    let mut count: i32 = 0;

    while count < n {
        if count == 0 {
            println!("0");
        } else if count == 1 {
            println!("1");
        } else {
            println!("{num}");
            p1 = p2;
            p2 = num;
            num = p1 + p2;
        }
        count += 1
    }
}

fn fibfor(n: i32) { // same as fibloop but using a for loop
    let mut p1: i32 = 0;
    let mut p2: i32 = 1;
    let mut num: i32 = p1 + p2;

    for count in 0..n {  // this is the rust "range"
        if count == 0 {
            println!("0");
        } else if count == 1 {
            println!("1");
        } else {
            println!("{num}");
            p1 = p2;
            p2 = num;
            num = p1 + p2;
        }
    }
}

fn main() {
    println!("Using 'loop'");
    fibloop(10);
    println!("Using 'while'");
    fibwhile(10);
    println!("Using 'for'");
    fibfor(10);
}
