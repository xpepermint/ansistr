use regex::Regex;

/// Regex pattern for maching ANSI codes.
pub const ANSI_REGEX: &'static str = r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]";

/// List of supported ANSI codes.
pub const ANSI_PAIR: [[&'static str; 2]; 24] = [
    ["\x1B[0m", "\x1B[0m"], // reset 0
    ["\x1B[1m", "\x1B[22m"], // bold 1
    ["\x1B[2m", "\x1B[22m"], // dim 2
    ["\x1B[3m", "\x1B[23m"], // italic 3
    ["\x1B[4m", "\x1B[24m"], // underline 4
    ["\x1B[5m", "\x1B[25m"], // blink 5
    ["\x1B[7m", "\x1B[27m"], // inverse 6
    ["\x1B[8m", "\x1B[28m"], // hidden 7
    ["\x1B[30m", "\x1B[39m"], // black 8
    ["\x1B[31m", "\x1B[39m"], // red 9
    ["\x1B[32m", "\x1B[39m"], // green 10
    ["\x1B[33m", "\x1B[39m"], // yellow 11
    ["\x1B[34m", "\x1B[39m"], // blue 12
    ["\x1B[35m", "\x1B[39m"], // magenta 13
    ["\x1B[36m", "\x1B[39m"], // cyan 14
    ["\x1B[37m", "\x1B[39m"], // white 15
    ["\x1B[40m", "\x1B[49m"], // bgblack 16
    ["\x1B[41m", "\x1B[49m"], // bgred 17
    ["\x1B[42m", "\x1B[49m"], // bggreen 18
    ["\x1B[43m", "\x1B[49m"], // bgyellow 19
    ["\x1B[44m", "\x1B[49m"], // bgblue 20
    ["\x1B[45m", "\x1B[49m"], // bgmagenta 21
    ["\x1B[46m", "\x1B[49m"], // bgcyan 22
    ["\x1B[47m", "\x1B[49m"], // bgwhite 23
];

/// Text alignement options.
#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    Left = 1,
    Center = 2,
    Right = 3,
}

/// Text alignement options.
#[derive(Debug, Clone, PartialEq)]
pub enum TextStyle {
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underlined = 4,
    Blinking = 5,
    Inversed = 6,
    Hidden = 7,
}

/// Text alignement options.
#[derive(Debug, Clone, PartialEq)]
pub enum TextColor {
    Black = 8,
    Red = 9,
    Green = 10,
    Yellow = 11,
    Blue = 12,
    Magenta = 13,
    Cyan = 14,
    White = 15,
}

/// Text alignement options.
#[derive(Debug, Clone, PartialEq)]
pub enum TextBackground {
    Black = 16,
    Red = 17,
    Green = 18,
    Yellow = 19,
    Blue = 20,
    Magenta = 21,
    Cyan = 22,
    White = 23,
}

fn ansi_pair<'a>(code: &'a str) -> Option<&[&str; 2]> {
    ANSI_PAIR.iter().find(|&pair| pair.iter().any(|&v| v == code))
}

/// Wraps text with ANSI style codes.
pub fn style_str<S: Into<String>>(txt: S, style: &TextStyle) -> String {
    let index = match style {
        TextStyle::Bold => 1,
        TextStyle::Dim => 2,
        TextStyle::Italic => 3,
        TextStyle::Underlined => 4,
        TextStyle::Blinking => 5,
        TextStyle::Inversed => 6,
        TextStyle::Hidden => 7,
    };
    format!("{}{}{}", ANSI_PAIR[index][0], txt.into(), ANSI_PAIR[index][1])
}

/// Wraps text with ANSI color codes.
pub fn color_str<S: Into<String>>(txt: S, color: &TextColor) -> String {
    let index = match color {
        TextColor::Black => 8,
        TextColor::Red => 9,
        TextColor::Green => 10,
        TextColor::Yellow => 11,
        TextColor::Blue => 12,
        TextColor::Magenta => 13,
        TextColor::Cyan => 14,
        TextColor::White => 15,
    };
    format!("{}{}{}", ANSI_PAIR[index][0], txt.into(), ANSI_PAIR[index][1])
}

/// Wraps text with ANSI background codes.
pub fn background_str<S: Into<String>>(txt: S, bg: &TextBackground) -> String {
    let index = match bg {
        TextBackground::Black => 16,
        TextBackground::Red => 17,
        TextBackground::Green => 18,
        TextBackground::Yellow => 19,
        TextBackground::Blue => 20,
        TextBackground::Magenta => 21,
        TextBackground::Cyan => 22,
        TextBackground::White => 23,
    };
    format!("{}{}{}", ANSI_PAIR[index][0], txt.into(), ANSI_PAIR[index][1])
}

/// Strips ANSI codes from text.
pub fn clean_str<S: Into<String>>(txt: S) -> String {
    let txt = txt.into();
    let regex = Regex::new(ANSI_REGEX).unwrap();
    let clean = String::from_utf8(regex.replace_all(&txt, "").as_bytes().to_vec());
    if clean.is_ok() {
        clean.unwrap()
    } else {
        txt
    }
}

pub fn match_indices<S: Into<String>>(txt: S) -> Vec<String> {
    let regex = Regex::new(ANSI_REGEX).unwrap();
    let mut result = Vec::new();
    let mut data: String = txt.into();

    loop {
        let mat = regex.find(data.as_str());
        if mat.is_some() {
            let mat = mat.unwrap();
            let start = mat.start();
            let end = mat.end();
            result.push(data[0..start].to_string());
            result.push(data[start..end].to_string());

            let size = data.chars().count();
            if size == 0 {
                break;
            } else {
                data = data[end..].to_string();
            }
        } else {
            result.push(data);
            break;
        }
    }

    result
}

pub fn slice_str<S: Into<String>>(txt: S, start: usize, end: usize) -> String {
    let mut u_start = None;
    let mut u_end = None;
    let mut offset = 0;
    let mut u_offset = 0;
    let txt = txt.into();

    for chunk in match_indices(&txt).iter() {
        let size = clean_str(chunk).len();
        
        if u_start.is_none() && offset + size >= start {
            u_start = Some(u_offset + start - offset);
        }
        if u_end.is_none() && offset + size >= end {
            u_end = Some(u_offset + end - offset);
            break;
        }
        offset += size;
        u_offset += chunk.len();
    }

    let u_start = match u_start {
        Some(v) => v,
        None => 0,
    };
    let u_end = match u_end {
        Some(v) => v,
        None => txt.len(),
    };
    txt[u_start..u_end].to_string()
}

pub fn size_str<S: Into<String>>(txt: S) -> usize {
    unicode_width::UnicodeWidthStr::width(clean_str(txt).as_str())
}

pub fn pad_str<S0: Into<String>, S1: Into<String>>(txt: S0, width: usize, align: &TextAlign, chr: S1) -> String {
    let txt = txt.into();
    let chr = chr.into();

    let size = size_str(&txt);
    if size >= width {
        return txt;
    }

    let chrsize = size_str(&chr);
    let diff = width - size;
    let (left_pad, right_pad) = match align {
        TextAlign::Left => (0, diff / chrsize),
        TextAlign::Right => (diff / chrsize, 0),
        TextAlign::Center => (diff / chrsize / 2, diff - diff / chrsize / 2),
    };

    let mut result = String::new();
    for _ in 0..left_pad {
        result.push_str(&chr);
    }
    result.push_str(&txt);
    for _ in 0..right_pad {
        result.push_str(&chr);
    }
    result
}

pub fn trucate_str<S0: Into<String>, S1: Into<String>>(txt: S0, width: usize, align: &TextAlign, tail: S1) -> String {
    let txt = txt.into();
    let tail = tail.into();

    let size = size_str(&txt);
    if width >= size {
        return txt;
    }

    let t_size = size_str(&tail);
    match align {
        TextAlign::Left => {
            let text = slice_str(&txt, 0, width - t_size).trim().to_string();
            format!("{}{}", text, tail)
        },
        TextAlign::Right => {
            let text = slice_str(&txt, size - width + t_size, size).trim().to_string();
            format!("{}{}", tail, text)
        },
        TextAlign::Center => {
            let dim = (width - t_size) / 2;
            let left = slice_str(&txt, 0, dim).trim().to_string();
            let right = slice_str(&txt, size - width + t_size + dim, size).trim().to_string();
            format!("{}{}{}", left, tail, right)
        },
    }
}

pub fn wrap_str<S: Into<String>>(txt: S, width: usize) -> String {
    let mut result: Vec<String> = Vec::new();
    let txt = txt.into();

    for line in txt.lines() {
        let mut words: Vec<String> = Vec::new();
        let mut length = 0;

        for (wcount, word) in line.split(" ").enumerate() {
            let word_size = size_str(word);
            if length + word_size >= width && words.len() > 0 {
                result.push(words.join(" "));
                words =  Vec::new();
                length = 0;
            }
            length += word_size + if wcount > 0 { 1 } else { 0 }; // include spaces
            words.push(word.to_string());
        }

        if words.len() > 0 {
            result.push(words.join(" "));
        }
    }

    result.join("\n")
}

pub fn repaire_str<S: Into<String>>(txt: S) -> String {
    let mut ansis: Vec<Vec<String>> = Vec::new();
    let txt = txt.into();

    let lines: Vec<String> = txt.split("\n").map(|line| {
        let parts = match_indices(line);

        let mut result: Vec<String> = Vec::new();
        let ansiiter = &ansis;
        for ansi in ansiiter.into_iter() {
            result.push(ansi[0].to_string());
        }
        for part in parts.into_iter() {
            let pair = ansi_pair(part.as_str());
            if pair.is_some() {
                let pair = pair.unwrap();
                let opentag = pair[0].to_string();
                let closetag = pair[1].to_string();
                if part == opentag {
                    ansis.push(vec![opentag, closetag]);
                } else if part == closetag {
                    let index = ansis.iter().position(|a| a[1].to_string() == closetag);
                    if index.is_some() {
                        ansis.remove(index.unwrap());
                    }
                }
            }
            result.push(part.to_string());
        }
        let ansiiter = &ansis;
        for ansi in ansiiter.into_iter() {
            result.push(ansi[1].to_string());
        }
        result.join("")
    }).collect();

    lines.join("\n")
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_ansi_pair() {
        assert_eq!(ansi_pair(&ANSI_PAIR[0][1]), Some(&ANSI_PAIR[0]));
        assert_eq!(ansi_pair("foo"), None);
    }

    #[test]
    fn applies_ansi_style() {
        style_str("foo", &TextStyle::Bold);
        assert_eq!(
            style_str("foo", &TextStyle::Bold),
            format!("{}{}{}", "\x1B[1m", "foo", "\x1B[22m"),
        );
    }

    #[test]
    fn applies_ansi_color() {
        assert_eq!(
            color_str("foo", &TextColor::Red),
            format!("{}{}{}", "\x1B[31m", "foo", "\x1B[39m"),
        );
    }

    #[test]
    fn applies_ansi_background() {
        assert_eq!(
            background_str("foo", &TextBackground::Green),
            format!("{}{}{}", "\x1B[42m", "foo", "\x1B[49m"),
        );
    }

    #[test]
    fn strips_ansi_codes() {
        assert_eq!(clean_str("aaa\x1B[0mbbb\x1B[0mccc"), "aaabbbccc");
    }

    #[test]
    fn matches_ansi_indices() {
        assert_eq!(match_indices("This is\x1B[39m long"), vec!["This is", "\x1B[39m", " long"]);
        assert_eq!(match_indices("This is\x1B[39m long \x1B[46mtext for test"), vec!["This is", "\x1B[39m", " long ", "\x1B[46m", "text for test"]);
    }

    #[test]
    fn slices_ansi_str() {
        assert_eq!(slice_str("a\x1B[32maa\x1B[32mb\x1B[32mbb\x1B[32mcccdddeeefff", 5, 10), "b\x1B[32mcccd");
    }

    #[test]
    fn sizes_ansi_str() {
        assert_eq!(size_str("aaa\x1B[0mbbb\x1B[0mccc"), 9);
    }

    #[test]
    fn pads_ansi_str() {
        assert_eq!(pad_str("fo\x1B[39mobar", 10, &TextAlign::Left, "+"), "fo\x1B[39mobar++++");
        assert_eq!(pad_str("fo\x1B[39mobar", 10, &TextAlign::Right, "+"), "++++fo\x1B[39mobar");
        assert_eq!(pad_str("fo\x1B[39mobar", 10, &TextAlign::Center, "+"), "++fo\x1B[39mobar++");
        assert_eq!(pad_str("fo\x1B[39mobar", 10, &TextAlign::Left, "\x1B[39m+!"), "fo\x1B[39mobar\x1B[39m+!\x1B[39m+!");
    }

    #[test]
    fn truncates_ansi_str() {
        assert_eq!(trucate_str("fo\x1B[39mobarbaz", 5, &TextAlign::Left, "+"), "fo\x1B[39mob+");
        assert_eq!(trucate_str("fo\x1B[39mobarbaz", 5, &TextAlign::Right, "+++"), "+++az");
        assert_eq!(trucate_str("fo\x1B[39mobarbaz", 5, &TextAlign::Center, "+++"), "f+++z");
    }

    #[test]
    fn wraps_ansi_str() {
        assert_eq!(wrap_str("This is \x1B[39ma very long tekst for testing\x1B[39m only.", 10), vec![
            "This is \x1B[39ma",
            "very long",
            "tekst for",
            "testing\x1B[39m",
            "only."
        ].join("\n"));
    }

    #[test]
    fn repairs_multiline_ansi_str() {
        assert_eq!(repaire_str(&vec![
            "This is \x1B[31mlong",
            "string 利干 sample",
            "this is 利干 sample\x1B[39m long code",
        ].join("\n")), vec![
            "This is \x1B[31mlong\x1B[39m",
            "\x1B[31mstring 利干 sample\x1B[39m",
            "\x1B[31mthis is 利干 sample\x1B[39m long code",
        ].join("\n"));
    }
}
