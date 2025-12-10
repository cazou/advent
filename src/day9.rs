use crate::traits::AdventOfCode;
use anyhow::Result;
use std::{ops::RangeInclusive, str::FromStr, vec};

pub struct Day9;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

struct Grid {
    red_tiles: Vec<Point>,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    borders: Vec<(Point, Point)>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut red_tiles: Vec<Point> = vec![];
        let mut borders = vec![];
        let mut x_range = None;
        let mut y_range = None;
        for line in s.lines() {
            let vals: Vec<isize> = line
                .split(',')
                .map(|v| v.parse::<isize>().unwrap())
                .collect();

            if x_range.is_none() {
                x_range = Some(vals[0]..=vals[0]);
            }
            if y_range.is_none() {
                y_range = Some(vals[1]..=vals[1]);
            }

            if *x_range.as_ref().unwrap().start() > vals[0] {
                x_range = Some(vals[0]..=*x_range.unwrap().end());
            }
            if *x_range.as_ref().unwrap().end() < vals[0] {
                x_range = Some(*x_range.unwrap().start()..=vals[0]);
            }
            if *y_range.as_ref().unwrap().start() > vals[1] {
                y_range = Some(vals[1]..=*y_range.unwrap().end());
            }
            if *y_range.as_ref().unwrap().end() < vals[1] {
                y_range = Some(*y_range.unwrap().start()..=vals[1]);
            }
            if let Some(point) = red_tiles.last() {
                borders.push((
                    point.clone(),
                    Point {
                        x: vals[0],
                        y: vals[1],
                    },
                ));
            }
            red_tiles.push(Point {
                x: vals[0],
                y: vals[1],
            });
        }

        borders.push((
            red_tiles.last().unwrap().clone(),
            red_tiles.first().unwrap().clone(),
        ));

        Ok(Grid {
            red_tiles,
            x_range: x_range.unwrap(),
            y_range: y_range.unwrap(),
            borders,
        })
    }
}

impl Grid {
    fn cab_dist(t1: &Point, t2: &Point) -> isize {
        (t1.x - t2.x).abs() + (t1.y - t2.y).abs()
    }

    fn find_biggest_rect(&mut self) -> isize {
        let top_left_corner = Point::new(*self.x_range.start(), *self.y_range.start());
        let top_right_corner = Point::new(*self.x_range.end(), *self.y_range.start());
        let bottom_left_corner = Point::new(*self.x_range.start(), *self.y_range.end());
        let bottom_right_corner = Point::new(*self.x_range.end(), *self.y_range.end());

        self.red_tiles.sort_by(|a, b| {
            Self::cab_dist(a, &top_left_corner).cmp(&Self::cab_dist(b, &top_left_corner))
        });
        let top_left_tile = self.red_tiles.first().unwrap().clone();

        self.red_tiles.sort_by(|a, b| {
            Self::cab_dist(a, &top_right_corner).cmp(&Self::cab_dist(b, &top_right_corner))
        });
        let top_right_tile = self.red_tiles.first().unwrap().clone();

        self.red_tiles.sort_by(|a, b| {
            Self::cab_dist(a, &bottom_left_corner).cmp(&Self::cab_dist(b, &bottom_left_corner))
        });
        let bottom_left_tile = self.red_tiles.first().unwrap().clone();

        self.red_tiles.sort_by(|a, b| {
            Self::cab_dist(a, &bottom_right_corner).cmp(&Self::cab_dist(b, &bottom_right_corner))
        });
        let bottom_right_tile = self.red_tiles.first().unwrap().clone();

        let r1 = ((top_left_tile.x - bottom_right_tile.x).abs() + 1)
            * ((top_left_tile.y - bottom_right_tile.y).abs() + 1);
        let r2 = ((top_right_tile.x - bottom_left_tile.x).abs() + 1)
            * ((top_right_tile.y - bottom_left_tile.y).abs() + 1);

        r1.max(r2)
    }

    fn segment_in_rect(seg: &(Point, Point), rect: &(Point, Point)) -> bool {
        let horiz;
        let vert;

        if seg.0.x == seg.1.x {
            if seg.0.x == rect.0.x || seg.0.x == rect.1.x {
                return false;
            }
        }

        if seg.0.y == seg.1.y {
            if seg.0.y == rect.0.y || seg.0.y == rect.1.y {
                return false;
            }
        }

        let left = seg.0.y.min(seg.1.y);
        let right = seg.0.y.max(seg.1.y);
        let left_rect = rect.0.y.min(rect.1.y);
        let right_rect = rect.0.y.max(rect.1.y);
        horiz = left < right_rect && right > left_rect;

        let up = seg.0.x.min(seg.1.x);
        let down = seg.0.x.max(seg.1.x);
        let up_rect = rect.0.x.min(rect.1.x);
        let down_rect = rect.0.x.max(rect.1.x);
        vert = up < down_rect && down > up_rect;

        horiz && vert
    }

    fn area(rect: &(Point, Point)) -> isize {
        ((rect.0.x - rect.1.x).abs() + 1) * ((rect.0.y - rect.1.y).abs() + 1)
    }

    fn find_biggest_green_rect(&mut self) -> isize {
        // Generate all rects
        let mut new_points = vec![];
        let mut rects = vec![];
        while !self.red_tiles.is_empty() {
            let p1 = self.red_tiles.remove(0);
            for p in &self.red_tiles {
                rects.push((p1, p.clone()));
            }

            new_points.push(p1);
        }

        // Find valid rectangles
        let mut valid_rects = vec![];
        while !rects.is_empty() {
            let rect = rects.remove(0);
            let mut valid = true;
            for seg in &self.borders {
                if Self::segment_in_rect(seg, &rect) {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_rects.push(rect.clone());
            }
        }

        // Find the biggest one
        valid_rects.sort_by(|r1, r2| Self::area(r2).cmp(&Self::area(r1)));

        Self::area(valid_rects.first().unwrap())
    }
}

impl AdventOfCode for Day9 {
    fn day(&self) -> u8 {
        9
    }

    fn run1(&mut self, input: Option<String>) -> Result<String> {
        let mut grid: Grid = input.unwrap().parse().unwrap();
        let biggest = grid.find_biggest_rect();
        Ok(biggest.to_string())
    }

    fn run2(&mut self, input: Option<String>) -> Result<String> {
        let mut grid: Grid = input.unwrap().parse().unwrap();
        let biggest = grid.find_biggest_green_rect();
        Ok(biggest.to_string())
    }
}
