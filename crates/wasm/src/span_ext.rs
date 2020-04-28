use neu_parser::TextRange;

#[derive(Clone, Copy)]
pub struct LineCols {
    pub line: i32,
    pub col_start: i32,
    pub col_end: i32
}

#[derive(Clone, Copy)]
pub struct LinesCols {
    pub line_start: i32,
    pub line_end: i32,
    pub col_start: i32,
    pub col_end: i32
}

pub trait TextRangeExt {
    fn line_cols(&self, lines: &[String]) -> Vec<LineCols>;
    fn lines_cols(&self, lines: &[String]) -> LinesCols;
}

impl TextRangeExt for TextRange {
    fn line_cols(&self, lines: &[String]) -> Vec<LineCols> {
        let start: usize = self.start().into();
        let end: usize = self.end().into();

        let (start_l, start_c) = build(start, lines);
        let (end_l, end_c) = build(end, lines);

        (start_l ..= end_l).map(|line| {
            let col_start = if line == start_l {
                start_c
            } else { 0_i32 };
            let col_end = if line == end_l {
                end_c
            } else {
                lines[line as usize].len() as i32
            };

            LineCols { line, col_start, col_end }
        }).collect()
    }

    fn lines_cols(&self, lines: &[String]) -> LinesCols {
        let start: usize = self.start().into();
        let end: usize = self.end().into();

        let (line_start, col_start) = build(start, lines);
        let (line_end, col_end) = build(end, lines);
        LinesCols { line_start, col_start, line_end, col_end }
    }
}

pub fn build(offset: usize, lines: &[String]) -> (i32, i32) {
    let mut off = offset;
    let mut line = 0_i32;
    let mut column = 0_i32;

    for (j, l) in lines.iter().enumerate() {
        line = j as i32;
        if off <= l.len() {
            column = off as i32;
            break;
        } else {
            off = off - l.len() - 1;
            line += 1_i32;
        }
    }

    ( line, column )
}
