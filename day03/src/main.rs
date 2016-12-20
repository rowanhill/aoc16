extern crate regex;

mod triangles;

use ::triangles::TRIANGLES_STR;
use std::str::SplitWhitespace;

#[derive(Copy, Clone)]
struct Triangle {
    a: u32,
    b: u32,
    c: u32
}

impl Triangle {
    fn new(a: u32, b: u32, c: u32) -> Triangle {
        Triangle { a: a, b: b, c: c }
    }

    fn is_possible(&self) -> bool {
        !(self.a + self.b <= self.c || self.a + self.c <= self.b || self.b + self.c <= self.a)
    }
}

// Iterator which takes an iterator of &str and turns them into an iterator of u32. If parsing the
// string into a number fails, it returns None
struct ParsedNumbersIterator<'a> {
    numbers: SplitWhitespace<'a>
}

impl<'a> ParsedNumbersIterator<'a> {
    fn new(numbers: SplitWhitespace) -> ParsedNumbersIterator {
        ParsedNumbersIterator {
            numbers: numbers
        }
    }
}

impl<'a> Iterator for ParsedNumbersIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.numbers.next().and_then(|a| a.parse::<u32>().ok())
    }
}


// Iterator which takes 3 u32s and turns them into Triangles. If any of the 3 u32s are None then the
// Triangle is None
struct RowTriangleIterator<'a> {
    numbers: ParsedNumbersIterator<'a>
}

impl<'a> RowTriangleIterator<'a> {
    fn new(numbers: SplitWhitespace) -> RowTriangleIterator {
        RowTriangleIterator {
            numbers: ParsedNumbersIterator::new(numbers)
        }
    }
}

impl<'a> Iterator for RowTriangleIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        self.numbers.next()
        .and_then(|a| self.numbers.next().map(|b| (a, b)))
        .and_then(|(a, b)| self.numbers.next().map(|c| (a, b, c)))
        .map(|(a, b, c)| Triangle::new(a, b, c))
    }
}


// Iterator which takes 3 Triangles and flips them from row-based to col-based. If any of the 3
// Triangles are None then all 3 flipped triangles are None
struct ColTriangleIterator<'a> {
    triangles: RowTriangleIterator<'a>,
    buffer: [Option<Triangle>; 3],
    buffer_index: usize
}

impl<'a> ColTriangleIterator<'a> {
    fn new(numbers: SplitWhitespace) -> ColTriangleIterator {
        ColTriangleIterator {
            triangles: RowTriangleIterator::new(numbers),
            buffer: [None; 3],
            buffer_index: 2
        }
    }
}

impl<'a> Iterator for ColTriangleIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_index == 2 {
            let three_triangles = self.triangles.next()
                .and_then(|a| self.triangles.next().map(|b| (a, b)))
                .and_then(|(a, b)| self.triangles.next().map(|c| (a, b, c)));

            self.buffer = match three_triangles {
                Some((a, b, c)) => [
                    Some(Triangle::new(a.a, b.a, c.a)),
                    Some(Triangle::new(a.b, b.b, c.b)),
                    Some(Triangle::new(a.c, b.c, c.c))
                ],
                None => [None; 3]
            };
        }

        self.buffer_index = (self.buffer_index + 1) % 3;

        self.buffer[self.buffer_index]
    }
}

fn main() {
    let row_iter = RowTriangleIterator::new(TRIANGLES_STR.split_whitespace());
    println!("Possible row triangles: {}", row_iter.filter(|triangle| triangle.is_possible()).count());

    let col_iter = ColTriangleIterator::new(TRIANGLES_STR.split_whitespace());
    println!("Possible col triangles: {}", col_iter.filter(|triangle| triangle.is_possible()).count());
}
