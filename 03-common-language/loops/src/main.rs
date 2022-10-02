/**
 * The keyword "loop" alone will execute the code block inside in an infinite
 * loop. The "break" keyword will cause the loop to exit. The "continue"
 * keyword will skip all remaining code in the code block and start the next
 * loop immediately
 */
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // this works in similar fashion as "return"
        }
    }; // this loop block will evaluate to the expression in the break stmt
    println!("The loop evaluates to {result}");
    
    let mut outercount: i32 = 0;
    let mut innercount: i32;

    'outerloop: loop { // the outer loop is given a label
        innercount = 0;
        loop {
            println!("Loc is ({outercount}, {innercount})");
            innercount +=1;

            if innercount >= 1000 { break; }  // this break the innert loop
            if outercount == 69 && innercount == 420 {
                println!("Nice!");
                break 'outerloop;  // this break out of the outer loop
            }
        }
        outercount += 1;
    }
    
    // TODO: what about while loop?
    // TODO: what about for loop?
}
