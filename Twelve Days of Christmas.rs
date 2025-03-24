const ORDINAL_NUMBERS: [&str; 12] = [
    "First", 
    "Second", 
    "Third",
    "Fourth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eighth",
    "Ninth",
    "Tenth",
    "Eleventh",
    "Twelfth"
];

const NEW_LYRICALS: [&str; 12] = [
    "And a partridge in a pear tree!",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    println!("🎄 The Twelve Days of Christmas 🎶");
    for i in 1..=12 {
        let mut day = i;
        println!("");
        println!("On the {} day of Christmas, my true love sent to me:", ORDINAL_NUMBERS[i - 1]);
        println!("");
        while day > 0 {
            if day == 1 && i == 1 {
                println!("A partridge in a pear tree!");
            } else {
                println!("{}", NEW_LYRICALS[day - 1]);
            }
            day -= 1; //Rust doesn't support ++ or --
        } 
        
    }
}