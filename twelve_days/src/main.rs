fn main() {
    let nums = [
        ["first", "a partridge in a pear tree"],
        ["second", "Two turtle doves"],
        ["third", "three French hens"],
        ["fourth", "Four calling birds"],
        ["fifth", "five gold rings"],
        ["sixth", "six geese a laying"],
        ["seventh", "Seven swans a swimming"],
        ["eighth", "eight maids a milking"],
        ["ninth", "nine ladies dancing"],
        ["tenth", "Ten lords a leaping"],
        ["eleventh", "eleven pipers piping"],
        ["twelfth", "Twelve drummers drumming"] 
    ];
    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me",
            nums[day][0]
        );
       for verse in (0..day+1).rev(){
        if day != 0 && verse == 0{
            print!("and ");
        }
            println!("{}", nums[verse][1]);
       }
       println!("=========================");
    }

}
