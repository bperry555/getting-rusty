fn main() {

    let days: [&str; 12] = ["First", "Second", "Third", "Forth", "Fifth", "Sixth",
                            "Seventh", "Eight", "Nineth", "Tenth", "Eleventh", "Twelfth"];



    let phrase: [&str; 12] =["A partridge in a pear tree", "Two turtle doves, and ", "Three french hens",
                        "Four calling birds", "Five golden rings", "Six geese a-laying", 
                        "Seven swans a-swimming", "Eight maids a-milking", "Nine Ladies Dancing",
                        "Ten Lords a Leaping", "Eleven Pipers Piping", "12 Drummers Drumming"];

    for day in 0..12 {
        println!("On the {} day of Christmas\nmy true love sent to me:", &days[day]);
        for item in (0..day + 1).rev() {
            println!("{}", &phrase[item]);
        }
    println!("\r");
    }
} //Main
