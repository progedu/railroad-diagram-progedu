use crate::Node;
use crate::notactuallysvg as svg;
use crate::notactuallysvg::HDir;

#[derive(Debug, Clone, Default)]
pub struct ProgEduStart;

impl Node for ProgEduStart {
    fn entry_height(&self) -> i64 {
        5
    }
    fn height(&self) -> i64 {
        10
    }

    fn width(&self) -> i64 {
        8
    }

    fn draw(&self, x: i64, y: i64, h_dir: HDir) -> svg::Element {
        svg::PathData::new(h_dir)
            .move_to(x, y)
            .vertical(10)
            .line_rel(8, -5)
            .line_rel(-8, -5)
            .vertical(5) // necessary because the library lacks the command to close the path
            .into_path()
            .debug("MyStart", x, y, self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProgEduEnd;

impl Node for ProgEduEnd {
    fn entry_height(&self) -> i64 {
        5
    }
    fn height(&self) -> i64 {
        10
    }

    fn width(&self) -> i64 {
        8
    }

    fn draw(&self, x: i64, y: i64, h_dir: HDir) -> svg::Element {
        svg::Element::new("g").set("class", "my-end").add(
            svg::PathData::new(h_dir)
                .move_to(x, y)
                .vertical(10)
                .line_rel(8, -5)
                .line_rel(-8, -5)
                .vertical(5) // necessary because the library lacks the command to close the path
                .into_path()
                .debug("MyEnd", x, y, self),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ForcedArrowRight;

impl Node for ForcedArrowRight {
    fn entry_height(&self) -> i64 {
        5
    }
    fn height(&self) -> i64 {
        10
    }

    fn width(&self) -> i64 {
        5
    }

    fn draw(&self, x: i64, y: i64, h_dir: HDir) -> svg::Element {
        svg::PathData::new(h_dir)
            .move_to(x, y)
			.move_rel(0, 5)
            .horizontal(5)
            .line_rel(-5, -5)
            .move_rel(0, 10)
            .line_rel(5, -5)
            .into_path()
            .debug("ForcedArrowRight", x, y, self)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ForcedArrowLeft;

impl Node for ForcedArrowLeft {
    fn entry_height(&self) -> i64 {
        5
    }
    fn height(&self) -> i64 {
        10
    }

    fn width(&self) -> i64 {
        5
    }

    fn draw(&self, x: i64, y: i64, h_dir: HDir) -> svg::Element {
        svg::PathData::new(h_dir)
            .move_to(x, y)
			.move_rel(0, 5)
            .horizontal(5)
			.move_rel(-5, 0)
            .line_rel(5, -5)
            .move_rel(0, 10)
            .line_rel(-5, -5)
            .into_path()
            .debug("ForcedArrowLeft", x, y, self)
    }
}