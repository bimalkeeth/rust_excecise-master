#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4)
}

#[derive()]
struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        3.14 * (self.radius * self.radius)
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }

    fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = Circle{
            radius:5.0,
        };

        let smaller_circle =Circle{
            radius:2.0,
        };

        assert!(larger_circle.contains(&smaller_circle),"failed");

    }
}