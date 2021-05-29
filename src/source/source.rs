use super::location;

pub struct Source<'src> {
    content: &'src str,
    lines: Vec<&'src str>,
}

impl<'src> Source<'src> {
    pub fn new(content: &str) -> Source {
        let mut lines: Vec<&str> = content.lines().collect();
        lines.push("");
        Source { content, lines }
    }

    pub fn chars(&self) -> std::str::Chars {
        self.content.chars()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn slice(&self, start: &location::Location, end: &location::Location) -> &str {
        &self.content[start.pos()..end.pos()]
    }

    pub fn pprint_loc(&self, loc: location::Location) {
        let row_start = std::cmp::max(loc.row(), 3) - 3;
        let row_end = loc.row() + 1;

        let mut digits: usize = 0;
        let mut base: usize = 1;
        while base < row_end {
            digits += 1;
            base *= 10;
        }

        let num = console::Style::new().dim();
        let caret = console::Style::new().red().bold();

        for i in row_start..row_end {
            eprintln!(
                " {:>digits$} {}",
                num.apply_to(i + 1),
                self.lines[i],
                digits = digits
            );
        }
        println!(
            " {:>digits$} {:>col$}",
            "",
            caret.apply_to("^"),
            digits = digits,
            col = loc.col() + 1,
        );
    }
}
