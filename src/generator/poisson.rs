use crate::generator::Point;
use rand::Rng;

type Sample = Point;

pub struct PoissonSampler {
    width: f32,
    height: f32,
    radius: f32,
    k: usize,
    cell_size: f32,
    grid: Grid,
    sample_list: Vec<Sample>,
    active_list: Vec<SampleIndex>,
}

// Two dimensional grid to hold sample indices
type Grid = Vec<Vec<SampleIndex>>;

type SampleIndex = i32;

const NO_SAMPLE: SampleIndex = -1;

impl PoissonSampler {
    pub fn new(width: f32, height: f32, radius: f32, k: usize) -> Self {
        let cell_size = radius / (2.0f32).sqrt();
        let grid = Self::create_grid(width, height, cell_size);
        let sample_list = Vec::new();
        let active_list = Vec::new();

        Self {
            width,
            height,
            radius,
            k,
            cell_size,
            grid,
            sample_list,
            active_list,
        }
    }

    fn create_grid(width: f32, height: f32, cell_size: f32) -> Grid {
        let grid_width = (width / cell_size).ceil() as usize;
        let grid_height = (height / cell_size).ceil() as usize;
        vec![vec![NO_SAMPLE; grid_height]; grid_width]
    }

    pub fn generate_samples(&mut self) -> Vec<Sample> {
        // Poisson disk sampling algorithm implementation
        // Returns a vector of Sample structs

        // 0. Initialize grid, active list, and samples list

        // 1. Generate initial sample
        let initial_sample = self.generate_initial_sample();
        self.insert_sample_into_grid(&initial_sample, 0);
        self.sample_list.push(initial_sample);
        self.active_list.push(0);

        while !self.active_list.is_empty() {
            let active_idx = rand::rng().random_range(0..self.active_list.len());
            let sample_idx = self.active_list[active_idx];
            let center = &self.sample_list[sample_idx as usize];

            let mut found = false;

            for _ in 0..self.k {
                let candidate = self.generate_point_in_annulus(center);
                if self.is_valid_sample(&candidate) {
                    let new_index = self.sample_list.len() as i32;
                    self.insert_sample_into_grid(&candidate, new_index);
                    self.sample_list.push(candidate);
                    self.active_list.push(new_index);
                    found = true;
                    break;
                }
            }

            if !found {
                self.active_list.swap_remove(active_idx);
            }
        }

        self.sample_list.clone()
    }

    fn insert_sample_into_grid(&mut self, sample: &Sample, index: i32) {
        let grid_x = (sample.x / self.cell_size).floor() as usize;
        let grid_y = (sample.y / self.cell_size).floor() as usize;
        self.grid[grid_x][grid_y] = index;
    }

    fn generate_initial_sample(&self) -> Sample {
        let x = rand::random::<f32>() * self.width;
        let y = rand::random::<f32>() * self.height;
        Sample { x, y }
    }

    fn generate_point_in_annulus(&self, sample: &Sample) -> Sample {
        let r1 = self.radius;
        let r2 = 2.0 * self.radius;
        let radius = ((rand::random::<f32>() * (r2 * r2 - r1 * r1)) + r1 * r1).sqrt();
        let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let x = sample.x + radius * angle.cos();
        let y = sample.y + radius * angle.sin();
        Sample { x, y }
    }

    fn is_in_bounds(&self, sample: &Sample) -> bool {
        sample.x >= 0.0 && sample.x < self.width && sample.y >= 0.0 && sample.y < self.height
    }

    fn is_valid_sample(&self, sample: &Sample) -> bool {
        if !self.is_in_bounds(sample) {
            return false;
        }

        let nearby_samples = self.get_nearby_samples(sample);
        for neighbor in nearby_samples {
            if Self::distance(sample, neighbor) < self.radius {
                return false;
            }
        }

        true
    }

    fn get_nearby_samples(&self, sample: &Sample) -> Vec<&Sample> {
        let mut nearby_samples = Vec::new();
        let grid_x = (sample.x / self.cell_size).floor() as isize;
        let grid_y = (sample.y / self.cell_size).floor() as isize;

        for i in -2..=2 {
            for j in -2..=2 {
                let neighbor_x = grid_x + i;
                let neighbor_y = grid_y + j;

                if neighbor_x >= 0
                    && neighbor_y >= 0
                    && (neighbor_x as usize) < self.grid.len()
                    && (neighbor_y as usize) < self.grid[0].len()
                {
                    let sample_index = self.grid[neighbor_x as usize][neighbor_y as usize];
                    if sample_index != NO_SAMPLE {
                        nearby_samples.push(&self.sample_list[sample_index as usize]);
                    }
                }
            }
        }

        nearby_samples
    }

    fn distance(a: &Sample, b: &Sample) -> f32 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_range(value: f32, min: f32, max: f32, msg: &str) {
        const TOLERANCE: f32 = 1e-4;

        assert!(
            value >= min - TOLERANCE && value <= max + TOLERANCE,
            "{}: {:.6} not in range [{}, {}]",
            msg,
            value,
            min,
            max
        );
    }

    #[test]
    fn test_minimum_distance_invariant() {
        let mut sampler = PoissonSampler::new(100.0, 100.0, 5.0, 30);
        let samples = sampler.generate_samples();

        for i in 0..samples.len() {
            for j in (i + 1)..samples.len() {
                let dist = PoissonSampler::distance(&samples[i], &samples[j]);
                assert_in_range(
                    dist,
                    5.0,
                    f32::INFINITY,
                    &format!("Samples {} and {} distance", i, j),
                );
            }
        }
    }

    #[test]
    fn test_samples_within_bounds() {
        let width = 100.0;
        let height = 100.0;

        let mut sampler = PoissonSampler::new(width, height, 5.0, 30);
        let samples = sampler.generate_samples();

        for (i, sample) in samples.iter().enumerate() {
            assert!(
                sample.x >= 0.0 && sample.x < width,
                "Sample {} x out of bounds",
                i
            );
            assert!(
                sample.y >= 0.0 && sample.y < height,
                "Sample {} y out of bounds",
                i
            );
        }
    }

    #[test]
    fn test_generates_at_least_one_sample() {
        let mut sampler = PoissonSampler::new(100.0, 100.0, 5.0, 30);
        let samples = sampler.generate_samples();
        assert!(!samples.is_empty(), "No samples generated");
    }

    #[test]
    fn test_annulus_generation() {
        let sampler = PoissonSampler::new(100.0, 100.0, 5.0, 30);
        let center = Sample { x: 50.0, y: 50.0 };

        for _ in 0..100 {
            let sample = sampler.generate_point_in_annulus(&center);
            let dist = PoissonSampler::distance(&center, &sample);
            assert_in_range(dist, 5.0, 10.0, "Annulus distance");
        }
    }
}
