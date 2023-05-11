use std::fmt::{self};

pub fn verse(n: u32) -> String {
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
        _ => {
            let mut suffix1 = "";
            let mut suffix2 = "s";
            if n > 1 {
                suffix1 = "s";
            }
            if n == 2 {
                suffix2 = "";
            }
            let mut v = 0;
            if n >= 1 {
                v = n - 1;
            }
            let mut finish = format!(
                "Take one down and pass it around, {0} bottle{1} of beer on the wall.\n",
                v, suffix2
            );
            if n == 1 {
                finish = format!(
                    "Take it down and pass it around, no more bottles of beer on the wall.\n"
                );
            }
            return format!(
                "{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.\n{2}",
                n, suffix1, finish
            );
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = "".to_owned();
    let mut i = start;
    while i >= end {
        song.push_str(&verse(i));
        if i > 0 {
            i = i - 1;
        } else {
            break;
        }
        if i >= end {
            song.push_str("\n")
        }
    }
    return song;
}
