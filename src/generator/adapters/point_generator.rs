pub trait PointGenerator {
    fn generate_points(&mut self) -> Vec<Point>;
}

impl PointGenerator for PoissonSampler {
    fn generate_points(&mut self) -> Vec<Point> {
        let samples = self.generate_samples();
        samples
            .into_iter()
            .map(|s| Point { x: s.x, y: s.y })
            .collect()
    }
}