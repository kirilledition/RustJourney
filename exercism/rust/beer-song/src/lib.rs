pub fn verse(n: u32) -> String {
    let mut beer_on_the_wall: String = format!("{n} bottles");
    let mut take_what_down: String = String::from("one");
    let beer_left;

    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n\n");
    };

    if n == 1 {
        beer_on_the_wall = String::from("1 bottle");
        take_what_down = String::from("it");
        beer_left = String::from("no more bottles")
    } else if n == 2 {
        beer_left = String::from("1 bottle");
    } else {
        beer_left = format!("{} bottles", n - 1);
    };

    format!("{beer_on_the_wall} of beer on the wall, {beer_on_the_wall} of beer.\nTake {take_what_down} down and pass it around, {beer_left} of beer on the wall.\n\n")
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1)).rev().map(verse).collect()
}
