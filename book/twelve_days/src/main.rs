const GREET: &str = "My true love sent to me";

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleveth", "twelvth",
    ];
    let presents = [
        "Twelve drummers drummin'",
        "Eleven pipers pipin'",
        "Ten lords a leapin'",
        "Nine ladies dancin'",
        "Eight maids milkin'",
        "Seven swans a swimmin'",
        "Six geese a layin'",
        "Five golden rings!",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves",
        "",
    ];
    let ending = [
        "And a partridge in a pear tree!",
        "A partridge in a pear tree!",
    ];
    let mut lyrics: String = "".to_string();
    for i in 0..12 {
        let mut end = if i == 0 { 1 } else { 0 };
        let mut present: String = "".to_string();
        for j in (11-i..11) {
            present.push_str(&format!("{}\n", presents[j].to_string().trim()).to_string());
        }
        let mut verse = construct_verse(
            presents[11 - i].to_string(),
            days[i].to_string(),
            present.to_string(),
            ending[end].to_string(),
        );
        lyrics.push_str(&verse);
    }
    println!("{}", lyrics);
}

fn construct_verse(opening: String, day: String, presents: String, ending: String) -> String {
    let mut verse: String = format!(
        "{}\nOn the {} day of Christmas\n{}\n{}{}\n\n",
        opening, day, GREET, presents, ending
    );
    verse
}
