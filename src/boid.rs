use Vector2d;

struct Boid {
    pos : Vector2d,
    dir: Vector2d,
    speed : f32,
    avoid_radius : f32,
    vision_radius : f32,
}

impl Boid {
    fn new(pos : Vector2d, dir: Vector2d, speed: f32) -> Boid {
        const AVOID_RADIUS : f32 = 5.0;
        const VISION_RADIUS : f32 = 100.0;

        Boid { pos: pos, dir: vel, speed: speed, avoid_radius: AVOID_RADIUS, vision_radius: VISION_RADIUS }
    }

    fn align(&mut self, boids : Vec<Boid>) {
        const ALIGNMENT_FACTOR : f32 = 0.7;

        // Filter boids to only those within the vision radius
        let mut boids_near = boids.iter()
        .filter(|boid| boid.pos.distance(self.pos) < self.vision_radius && boid != &self);

        // Calculate the average direction of the boids in the vision radius
        let mut boids_dir = boids_near.iter()
        .map(|boid| boid.dir).collect();

        // Apply the alignment factor to the current direction
        self.dir = ((1 - ALIGNMENT_FACTOR) * self.dir) + (ALIGNMENT_FACTOR * Vector2d::normalize(Vector2d::average(boids_dir)));
        Vector2d::normalize(&mut self.dir);

        // Apply the alignment factor to the current speed
        self.speed = ((1 - ALIGNMENT_FACTOR) * self.speed) + (ALIGNMENT_FACTOR * boids_near.iter().map(|boid| boid.speed).collect().average());
    }

    fn avoid(&mut self, boids : Vec<Boid>) {
        const AVOIDANCE_FACTOR : f32 = 0.1;

        // Filter boids to only those within the avoid radius
        let mut boids_near = boids.iter()
        .filter(|boid| boid.pos.distance(self.pos) < self.avoid_radius && boid != &self);

        // Calculate the average position of the boids in the avoid radius
        let mut boids_dir = boids_near.iter()
        .map(|boid| boid.pos).collect();

        self.dir = ((1 - AVOIDANCE_FACTOR) * self.dir) + (AVOIDANCE_FACTOR * Vector2d::normalize(self.pos - Vector2d::average(boids_dir)));
        Vector2d::normalize(&mut self.dir);
    }

    fn cohere(&mut self, boids : Vec<Boid>) {
        const COHESION_FACTOR : f32 = 0.1;

        // Filter boids to only those within the vision radius
        let mut boids_near = boids.iter()
        .filter(|boid| boid.pos.distance(self.pos) < self.vision_radius && boid != &self);

        // Calculate the average position of the boids in the avoid radius
        let mut boids_dir = boids_near.iter()
        .map(|boid| boid.pos).collect();

        self.dir = ((1 - COHESION_FACTOR) * self.dir) + (COHESION_FACTOR * Vector2d::normalize(Vector2d::average(boids_dir) - self.pos));
        Vector2d::normalize(&mut self.dir);
    }

    // fn update()
}

