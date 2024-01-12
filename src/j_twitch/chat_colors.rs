use colored::{ColoredString, Colorize};

#[derive(Debug)]
pub enum ChatColor {
    Blue = 0,
    Green = 1,
    Yellow = 2,
    Red = 3,
    Purple = 4,
    Cyan = 5,
    Magenta = 6,
    White = 7,
}

impl From<i32> for ChatColor {
    fn from(value: i32) -> Self {
        return match value {
            0 => ChatColor::Blue,
            1 => ChatColor::Green,
            2 => ChatColor::Yellow,
            3 => ChatColor::Red,
            4 => ChatColor::Purple,
            5 => ChatColor::Cyan,
            6 => ChatColor::Magenta,
            7 => ChatColor::White,
            _ => ChatColor::Blue,
        };
    }
}

impl ChatColor {
    pub fn to(s: &str, code: ChatColor) -> ColoredString {
        return match code {
            ChatColor::Blue => s.blue(),
            ChatColor::Green => s.green(),
            ChatColor::Yellow => s.yellow(),
            ChatColor::Red => s.red(),
            ChatColor::Purple => s.purple(),
            ChatColor::Cyan => s.cyan(),
            ChatColor::Magenta => s.magenta(),
            ChatColor::White => s.white(),
        };
    }
}
