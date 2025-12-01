#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Rectangle {
    pub fn make_by_coords(x1: f32, y1: f32, x2: f32, y2: f32) -> Rectangle {
        Rectangle { x1, y1, x2, y2 }
    }

    pub fn make_by_size(left: f32, bottom: f32, width: f32, height: f32) -> Rectangle {
        Rectangle {
            x1: left,
            y1: bottom,
            x2: left + width,
            y2: bottom + height,
        }
    }

    pub fn advance(&self, vector: &FVector2d) -> Rectangle {
        Rectangle {
            x1: self.x1 + vector.x,
            y1: self.y1 + vector.y,
            x2: self.x2 + vector.x,
            y2: self.y2 + vector.y,
        }
    }

    pub fn with_left_at(&self, new_left: f32) -> Rectangle {
        let old_left = self.left();
        let diff = new_left - old_left;
        Rectangle {
            x1: new_left,
            y1: self.y1,
            x2: new_left + f32::abs(self.x1 - self.x2),
            y2: self.y2,
        }
    }

    pub fn with_right_at(&self, new_right: f32) -> Rectangle {
        let old_right = self.right();
        let diff = new_right - old_right;
        Rectangle {
            x1: new_right - f32::abs(self.x1 - self.x2),
            y1: self.y1,
            x2: new_right,
            y2: self.y2,
        }
    }

    pub fn grow(&self, delta: f32) -> Rectangle {
        Rectangle {
            x1: self.left() - delta,
            y1: self.top() + delta,
            x2: self.right() + delta,
            y2: self.bottom() - delta,
        }
    }

    pub fn left(&self) -> f32 {
        f32::min(self.x1, self.x2)
    }

    pub fn right(&self) -> f32 {
        f32::max(self.x1, self.x2)
    }

    pub fn top(&self) -> f32 {
        f32::max(self.y1, self.y2)
    }

    pub fn bottom(&self) -> f32 {
        f32::min(self.y1, self.y2)
    }

    pub fn top_left(&self) -> FPoint {
        FPoint::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> FPoint {
        FPoint::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> FPoint {
        FPoint::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> FPoint {
        FPoint::new(self.right(), self.bottom())
    }

    pub fn mutable_set(&mut self, rect: Rectangle) {
        self.x1 = rect.x1;
        self.y1 = rect.y1;
        self.x2 = rect.x2;
        self.y2 = rect.y2;
    }
}

#[derive(Debug)]
pub struct ISize {
    pub h: i32,
    pub w: i32,
}

#[derive(Debug, Clone)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

impl FPoint {
    pub fn zero() -> FPoint {
        FPoint { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> FPoint {
        FPoint { x, y }
    }

    pub fn add(&self, other: FVector2d) -> FPoint {
        FPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn within_rectangle(&self, rectangle: &Rectangle) -> bool {
        self.x >= rectangle.left()
            && self.x <= rectangle.right()
            && self.y >= rectangle.bottom()
            && self.y <= rectangle.top()
    }

    pub fn sq_dist(point1: &FPoint, point2: &FPoint) -> f32 {
        (point1.x - point2.x) * (point1.x - point2.x)
            + (point1.y - point2.y) * (point1.y - point2.y)
    }

    pub fn is_same(&self, point2: &FPoint, error_margin: f32) -> bool {
        ((self.x - point2.x).abs() < error_margin) && ((self.y - point2.y).abs() < error_margin)
    }
}

#[derive(Debug, Clone)]
pub struct FVector2d {
    pub x: f32,
    pub y: f32,
}

impl FVector2d {
    pub fn new(x: f32, y: f32) -> FVector2d {
        FVector2d { x, y }
    }

    pub fn zero() -> FVector2d {
        FVector2d::new(0.0, 0.0)
    }

    pub fn between(from: &FPoint, to: &FPoint) -> FVector2d {
        FVector2d::new(to.x - from.x, to.y - from.y)
    }

    pub fn get_normal(&self) -> FVector2d {
        FVector2d::new(self.y, -self.x)
    }

    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_square().sqrt()
    }

    pub fn invert(mut self) -> FVector2d {
        self.x = -self.x;
        self.y = -self.y;
        return self;
    }

    pub fn plus(mut self, other: &FVector2d) -> FVector2d {
        self.x += other.x;
        self.y += other.y;
        return self;
    }

    pub fn minus(mut self, other: &FVector2d) -> FVector2d {
        self.x -= other.x;
        self.y -= other.y;
        return self;
    }

    pub fn mul_scalar(mut self, scalar: f32) -> FVector2d {
        self.x *= scalar;
        self.y *= scalar;
        return self;
    }

    pub fn dot_product(&self, other: &FVector2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(mut self) -> FVector2d {
        let length = self.length();
        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }
        return self;
    }

    pub fn reflect(ray: &FVector2d, surface_normal: &FVector2d) -> FVector2d {
        let normalized_normal = surface_normal.clone().normalize();
        let dot = ray.dot_product(&normalized_normal);
        let result = ray.clone()
            .minus(&normalized_normal.mul_scalar(2.0 * dot));
        return result;
    }

    pub fn rotate_clockwise(mut self, clockwise_angle: f32) -> FVector2d {
        let (sn, cs) = f32::sin_cos(-clockwise_angle);
        let new_x = self.x * cs - self.y * sn;
        let new_y = self.x * sn + self.y * cs;
        self.x = new_x;
        self.y = new_y;
        return self;
    }

    pub fn is_zero(&self) -> bool {
        f32::abs(self.x) < f32::EPSILON && f32::abs(self.y) < f32::EPSILON
    }
}


#[derive(Debug, Clone)]
pub struct Circle {
    pub center: FPoint,
    pub radius: f32
}

impl Circle {
    pub fn new(center: FPoint, radius: f32) -> Circle {
        Circle { center, radius }
    }
}

#[derive(Debug)]
pub struct Collision {
    pub point: FPoint,
    pub normal: FVector2d
}

pub fn circle_rectangle_collision(circle: &Circle, rectangle: &Rectangle) -> Option<Collision> {
    let radius_sq = circle.radius * circle.radius;
    let collistion_point_opt = if FPoint::new(circle.center.x + circle.radius, circle.center.y).within_rectangle(rectangle) {
        Some(FPoint::new(
            rectangle.left(),
            circle.center.y,
        ))
    } else if FPoint::new(circle.center.x - circle.radius, circle.center.y).within_rectangle(rectangle) {
        Some(FPoint::new(
            rectangle.right(),
            circle.center.y,
        ))
    } else if FPoint::new(circle.center.x, circle.center.y + circle.radius).within_rectangle(rectangle) {
        Some(FPoint::new(
            circle.center.x,
            rectangle.bottom(),
        ))
    } else if FPoint::new(circle.center.x, circle.center.y - circle.radius).within_rectangle(rectangle) {
        Some(FPoint::new(
            circle.center.x,
            rectangle.top(),
        ))
    } else if FPoint::sq_dist(&circle.center, &rectangle.bottom_left()) < radius_sq {
        Some(rectangle.bottom_left())
    } else if FPoint::sq_dist(&circle.center, &rectangle.bottom_right()) < radius_sq {
        Some(rectangle.bottom_right())
    } else if FPoint::sq_dist(&circle.center, &rectangle.top_left()) < radius_sq {
        Some(rectangle.top_left())
    } else if FPoint::sq_dist(&circle.center, &rectangle.top_right()) < radius_sq {
        Some(rectangle.top_right())
    } else {
        None
    };

    collistion_point_opt.map(|point| {
        let normal = FVector2d::between(&point, &circle.center);
        Collision { point, normal }
    })
}

pub fn new_vector_after_circle_collision(circle_movement_vector: &FVector2d, collision: &Collision, other_movement_vector: &FVector2d) -> FVector2d {
    let normalized_normal = collision.normal.clone().normalize();
    let normal_movement_dot_product = circle_movement_vector.dot_product(&normalized_normal);
    if normal_movement_dot_product >= 0.0 {
        circle_movement_vector.clone()
            .plus(other_movement_vector)
    } else {
        circle_movement_vector.clone()
            .minus(&normalized_normal.mul_scalar(2.0 * normal_movement_dot_product))
    }
}
