
#[derive(Copy, Clone)]

pub struct Point {
    x: f32,
    y: f32
}

impl Default for Point {
    fn default() -> Self {
        Point{x:0.0,y:1.0}
    }
}

impl Point {

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }


    pub fn set_xy(&mut self, x: f32 , y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn rotate_by(&mut self, angle: f32) {
        self.x = self.x * angle.cos() - self.y * angle.sin();
        self.y = self.y * angle.cos() + self.x * angle.sin();
    }

    pub fn x(&self) -> f32 {
        return self.x;
    }

    pub fn y(&self) -> f32 {
        return self.y;
    }

    pub fn xy(&self) -> [f32; 2] {
        [self.x, self.y ]
    }

    pub fn normalise_self_to (&mut self, desired_length: f32) {
        let coef = desired_length / self.size();
       // let mut new_vec = self; //Point3D::default();
        self.x = self.x * coef;
        self.y = self.y * coef;
    }
    pub fn scale_vector(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
    pub fn add(&mut self, point: &Point)  {
        self.x += point.x;
        self.y += point.y;
    }

    pub fn size(&self) -> f32 {
        ( self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance_to(&self, point: &Point) -> f32 {
        ( ( self.x - point.x ).powi(2) + (  self.y - point.y ).powi(2)  ).sqrt()
    }

    pub fn taxicab_size(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    // asfasf
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y }
    }

}