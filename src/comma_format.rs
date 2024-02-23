use std::fmt::Display;

pub struct CommaFormat(pub usize);

impl Display for CommaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = std::iter::successors(Some(self.0), |&k| (k > 1000).then_some(k / 1000))
            .collect::<Vec<_>>();

        if let Some(width) = f.width() {
            for _ in 0..width.saturating_sub(o.len() * 4 + 1) {
                write!(f, "{}", f.fill())?;
            }
        }

        for (index, number) in o.iter().rev().enumerate() {
            if index == 0 {
                write!(f, "{:>3}", number % 1000)?;
            } else {
                write!(f, ",{:03}", number % 1000)?;
            }
        }
        Ok(())
    }
}
