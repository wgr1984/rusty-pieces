use tui::widgets::canvas::Shape;
use tui::style::Color;

pub struct Cricle {
    pub radius: f64,
    pub center: (f64, f64),
    pub color: Color,
}


impl Cricle {

}

pub struct CircleIterator {
    center: (f64, f64),
    step: u32,
    nr_of_points: u32,
    radius: f64,
}

impl Iterator for CircleIterator {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {

        if self.step > self.nr_of_points {
            return None;
        }

        let next_angle = ((self.step as f64 / self.nr_of_points as f64) * 360_f64).to_radians();

        self.step += 1;

        let point = ((next_angle.cos() * self.radius) + self.center.0, (next_angle.sin() * self.radius) + self.center.1);

        Some(point)
    }
}

impl<'a> IntoIterator for &'a Cricle {
    type Item = (f64, f64);
    type IntoIter = CircleIterator;

    fn into_iter(self) -> Self::IntoIter {
        CircleIterator {
            center: self.center,
            radius: self.radius,
            nr_of_points: (std::f64::consts::PI * self.radius * 2_f64).round() as u32,
            step: 0,
        }
    }
}

impl<'a> Shape<'a> for Cricle {
    fn color(&self) -> Color {
        self.color
    }

    fn points(&'a self) -> Box<dyn Iterator<Item = (f64, f64)> + 'a> {
        Box::new(self.into_iter())
    }
}