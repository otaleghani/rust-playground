fn main() {
    let days = [
       String::from("first"), 
       String::from("second"), 
       String::from("third"), 
       String::from("fourth"), 
       String::from("fifth"), 
       String::from("sixth"), 
       String::from("seventh"), 
       String::from("eighth"), 
       String::from("ninth"), 
       String::from("tenth"), 
       String::from("eleventh"), 
       String::from("twelfth"), 
    ];

    let gift = [
        String::from("A partridge in a pear tree"),
        String::from("Two turtle doves"), 
        String::from("Three French hens"), 
        String::from("Four calling birds"), 
        String::from("Five golden rings"), 
        String::from("Six geese a-laying"), 
        String::from("Seven swans a-swimming"), 
        String::from("Eight maids a-milking"), 
        String::from("Nine ladies dancing"), 
        String::from("Ten lords a-leaping"), 
        String::from("Eleven pipers piping"), 
        String::from("Twelve drummers drumming"), 
    ];

    let mut base = String::from("day of Christmas,\nmy true love gave to me");

    let mut body = String::from("\n");

    for i in 0..days.len() {
        if i == 0 {
            body = body + &gift[i];
        } else {
            body = body + &String::from(",\n") + &gift[i];
        }
        let title = String::from("On the ") + &days[i] + &String::from(" day of Christmas,\nmy true love gave to me");

        base = String::from("On the ") + &days[i] + &base + &gift[i];

        if i == days.len() - 1 {
            println!("{title}{body}!\n");
        } else {
            println!("{title}{body}.\n");
        }
        // body = body + &String::from("\n");
    }
}
