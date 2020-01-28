pub fn verse(n: u32) -> String {
    if n < 1 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n", 99);
    }
    if n == 1 {
        return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
    }

    let mut bottle = "bottles";
    if n - 1 < 2 {
        bottle = "bottle";
    }
    return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, n, n - 1, bottle);
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for (i, current) in (end..start + 1).enumerate() {exercism download --exercise=proverb --track=rust
        let current = start - i as u32;
        song += &verse(current);
        if current != end {
            song += &"\n";
        }
    }
    return song;
}
