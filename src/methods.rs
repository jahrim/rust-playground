/// # Implementations (~ Java Methods)
struct Point {
    /// ## Fields
    x: f64,
    y: f64
}

impl Point {
    /// ## Static Field
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };

    /// ## Static Method
    fn new(x: f64, y: f64) -> Point { Point { x: x, y: y } }

    /// ## Immutable Method
    /// Creates a new instance of `Self`
    fn translate(
        self: &Self,    // self/this type (here is the same as `Point`)
        dx: f64,
        dy: f64,
    ) -> Point {
        /// ## Static Method/Field Access `::`
        Point::new(self.x + dx, self.y + dy)    
    }

    fn translate_x(
        &self,          // sugar for `self: &Self`
        dx: f64,
    ) -> Point {
        /// ## Method/Field Access `.`
        self.translate(dx, 0.0)                 
    }

    /// ## Mutable Method
    /// Updates `self` (only callable by mutable variables)
    fn translate_mutable(
        self: &mut Self,
        dx: f64,
        dy: f64,
    ) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }   

    fn translate_x_mutable(
        &mut self,      // sugar for `self: &mut Self`
        dx: f64,
    ) {
        self.translate_mutable(dx, 0.0)
    }   

    /// ## Consuming Method
    /// Consumes the object after returning, so it cannot be referenced anymore
    fn destroy1(self: Self) {
        println!("destroy1: Point({}, {})", self.x, self.y)
        // After returning,  `self` is destroyed (aka null)
    }

    fn destroy2(self) { // sugar for self: Self
        println!("destroy2: Point({}, {})", self.x, self.y)
        // After returning, `self` is destroyed (aka null)
    }
}

runnable!(methods, {
    let point: Point = Point::new(0.0, 0.0);
    let point: Point = point.translate(10.0, 5.0);
    // point.translate_mutable(1.0, 1.0);
    // ^ Error: `point` is immutable
    point.destroy1();
    // point.destroy2(); 
    // ^ Error: `point` cannot be accessed anymore

    let mut point_mut: Point = Point::new(0.0, 0.0);
    let mut point_mut: Point = point_mut.translate(10.0, 5.0);
    point_mut.translate_mutable(0.0, 5.0);
    point_mut.destroy1();
});