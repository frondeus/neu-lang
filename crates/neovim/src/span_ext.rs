use neu_parser::TextRange;

#[derive(Clone, Copy)]
pub struct LineCols {
    pub line: i64,
    pub col_start: i64,
    pub col_end: i64
}

pub trait TextRangeExt {
    fn lines_cols(&self, lines: &[String]) -> Vec<LineCols>;
}

impl TextRangeExt for TextRange {
    fn lines_cols(&self, lines: &[String]) -> Vec<LineCols> {
        let start: usize = self.start().into();
        let end: usize = self.end().into();

        let (start_l, start_c) = build(start, lines);
        let (end_l, end_c) = build(end, lines);

        (start_l ..= end_l).map(|line| {
            let col_start = if line == start_l {
                start_c
            } else { 0_i64 };
            let col_end = if line == end_l {
                end_c
            } else {
                lines[line as usize].len() as i64
            };

            LineCols { line, col_start, col_end }
        }).collect()
    }
}

pub fn build(offset: usize, lines: &[String]) -> (i64, i64) {
    let mut off = offset;
    let mut line = 0_i64;
    let mut column = 0_i64;

    for (j, l) in lines.iter().enumerate() {
        line = j as i64;
        if off <= l.len() {
            column = off as i64;
            break;
        } else {
            off = off - l.len() - 1;
            line += 1_i64;
        }
    }

    ( line, column )
}
