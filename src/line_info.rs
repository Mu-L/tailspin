#[derive(Default)]
pub struct LineInfo {
    pub colons: usize,
    pub dashes: usize,
    pub dots: usize,
    pub double_quotes: usize,
    pub equals: usize,
    pub slashes: usize,
}

impl LineInfo {
    pub fn process(line: &str) -> LineInfo {
        let mut colons = 0;
        let mut dashes = 0;
        let mut dots = 0;
        let mut double_quotes = 0;
        let mut equals = 0;
        let mut slashes = 0;

        for c in line.chars() {
            match c {
                ':' => colons += 1,
                '-' => dashes += 1,
                '.' => dots += 1,
                '"' => double_quotes += 1,
                '=' => equals += 1,
                '/' => slashes += 1,
                _ => {}
            }
        }

        LineInfo {
            slashes,
            dots,
            dashes,
            double_quotes,
            equals,
            colons,
        }
    }
}
