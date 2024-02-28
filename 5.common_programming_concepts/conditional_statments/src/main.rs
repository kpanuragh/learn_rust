fn main() {
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }
    println!("Match instead of multiple if else");
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    let day = Day::Friday;
    match day {
        Day::Monday => println!("Start of the work week."),
        Day::Tuesday => println!("Second day of the work week."),
        Day::Wednesday => println!("Midweek."),
        Day::Thursday => println!("Almost there."),
        Day::Friday => println!("End of the work week."),
        Day::Saturday | Day::Sunday => println!("Weekend! Time to relax."),
    }
    println!("For loop example:");
    for x in 1..10 { //1...10 is range of 1 to 9 
        println!("{}", x);
    }
    println!("For loop example with index:");
    // index with value in range
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
    println!("For loop example with step by 2:");
    //step by 2
    for x in (1..10).step_by(2) {
        println!("{}", x);
    }
    println!("For loop example with inclusive range:");
    // include uper bound
    for x in 1..=10 {
        println!("{}", x);
    }
    println!("While loop example:");
    //while loop
    let mut x = 5;
    while x != 0 {
        println!("{}", x);
        x -= 1;
    }
}
