
use crate::geometry::*;

const EPSILON: f32 = 10e-6;

#[test]
fn circle_rectangle_collision_no_collision() {
    let circle = Circle::new(FPoint::new(100.0, 100.0), 10.0);
    let rect = Rectangle::make_by_coords(10.0, 10.0, 20.0, 20.0);

    assert_eq!(circle_rectangle_collision(&circle, &rect).is_some(), false);
}

#[test]
fn circle_rectangle_collision_flat_collisions() {
    let rect = Rectangle::make_by_coords(0.0, 0.0, 100.0, 100.0);
    let tests = vec!(
        ("Top collistion", Circle::new(FPoint::new(50.0, 110.0), 11.0), FPoint::new(50.0, 100.0)),
        ("Bottom collision", Circle::new(FPoint::new(50.0, -10.0), 11.0), FPoint::new(50.0, 0.0)),
        ("Left collision", Circle::new(FPoint::new(-10.0, 50.0), 11.0), FPoint::new(0.0, 50.0)),
        ("Right collision", Circle::new(FPoint::new(110.0, 50.0), 11.0), FPoint::new(100.0, 50.0)),
    );

    for (test_name, circle, expected_collision_point) in tests.iter() {
        let result = circle_rectangle_collision(&circle, &rect);

        assert!(matches!(&result,
                         Some(CollisionKind::Flat(point)) if point.is_same(expected_collision_point, EPSILON)),
                "Test failed {:?}. Expected: {:?}. Actual: {:?}", test_name, expected_collision_point, result);
    }
}

#[test]
fn circle_rectangle_collision_corner_collisions() {
    let rect = Rectangle::make_by_coords(0.0, 0.0, 100.0, 100.0);
    let tests = vec!(
        ("TopLeft collistion", Circle::new(FPoint::new(-7.0, 107.0), 11.0), FPoint::new(0.0, 100.0)),
        ("TopRight collision", Circle::new(FPoint::new(107.0, 107.0), 11.0), FPoint::new(100.0, 100.0)),
        ("BottomLeft collision", Circle::new(FPoint::new(-7.0, -7.0), 11.0), FPoint::new(0.0, 0.0)),
        ("BottomRight collision", Circle::new(FPoint::new(107.0, -7.0), 11.0), FPoint::new(100.0, 0.0)),
    );

    for (test_name, circle, expected_collision_point) in tests.iter() {
        let result = circle_rectangle_collision(&circle, &rect);

        assert!(matches!(&result,
                         Some(CollisionKind::Corner(point)) if point.is_same(expected_collision_point, EPSILON)),
                "Test failed {:?}. Expected: {:?}. Actual: {:?}", test_name, expected_collision_point, result);
    }
}
