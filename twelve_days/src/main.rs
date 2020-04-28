fn main() {
    let nums = [
        ["first", "a partridge in a pear tree"],
        ["second", "turtle doves,"],
        ["third", "French hens,"],
        ["fourth", "calling birds,"],
        ["fifth", "gold rings,"],
        ["sixth", "geese a laying,"],
        ["seventh", "swans a swimming,"],
        ["eighth", "maids a milking,"],
        ["ninth", "ladies dancing,"],
        ["tenth", "lords a leaping,"],
        ["eleventh", "pipers piping,"],
        ["twelfth", "drummers drumming,"] 
    ];

    let numbers = [
        "",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve"
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me",
            nums[day][0]
        );
       for verse in (0..day+1).rev(){
        if day != 0 && verse == 0{
            print!("and");
        }
            println!("{} {}", numbers[verse], nums[verse][1]);
       }
       println!("=========================\n");
    }

}
