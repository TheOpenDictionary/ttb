use console::{style, Emoji};

pub struct Stepper {
    total: u64,
    current: u64,
}

impl Stepper {
    pub fn new(steps: u64) -> Stepper {
        Stepper {
            total: steps,
            current: 1,
        }
    }

    pub fn print_step(&mut self, emoji: &str, label: &str) {
        if self.current <= self.total {
            let emoji_prefix = &format!("{} ", emoji);
            let emoji_ = Emoji(emoji_prefix, "");
            let nums = format!("[{}/{}]", self.current, self.total);
            let prefix = style(nums).bold().dim();

            println!("{} {}{}", prefix, emoji_, label);

            self.current += 1;
        }
    }
}
