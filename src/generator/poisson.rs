use crate::config::required_distance;
use crate::generator::Point;
use crate::generator::Sample;

pub struct PoissonSampler {
    width: f32,
    height: f32,
    min_radius: u32,
    max_radius: u32,
    spacing_multiplier: f32,
    k: usize,
    cell_size: f32,
    grid_search_range: isize,
    grid: Grid,
    sample_list: Vec<Sample>,
    active_list: Vec<SampleIndex>,
}

type Grid = Vec<Vec<Vec<SampleIndex>>>;

type SampleIndex = i32;

impl PoissonSampler {
    pub fn new(
        width: f32,
        height: f32,
        min_radius: u32,
        max_radius: u32,
        spacing_multiplier: f32,
        k: usize,
    ) -> Self {
        let maximum_interaction_distance = 2.0 * max_radius as f32 * spacing_multiplier;
        let cell_size = maximum_interaction_distance / (2.0f32).sqrt();
        let grid_search_range = (maximum_interaction_distance / cell_size).ceil() as isize;
        let grid = Self::create_grid(width, height, cell_size);
        let sample_list = Vec::new();
        let active_list = Vec::new();

        Self {
            width,
            height,
            min_radius,
            max_radius,
            spacing_multiplier,
            k,
            cell_size,
            grid_search_range,
            grid,
            sample_list,
            active_list,
        }
    }

    fn create_grid(width: f32, height: f32, cell_size: f32) -> Grid {
        let grid_width = (width / cell_size).ceil() as usize;
        let grid_height = (height / cell_size).ceil() as usize;
        vec![vec![Vec::new(); grid_height]; grid_width]
    }

    pub fn generate_samples(&mut self) -> Vec<Sample> {
        let initial_sample = self.generate_initial_sample();
        self.insert_sample_into_grid(&initial_sample, 0);
        self.sample_list.push(initial_sample);
        self.active_list.push(0);

        while !self.active_list.is_empty() {
            let active_idx = rand::random_range(0..self.active_list.len());
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
        let grid_x = (sample.position.x / self.cell_size).floor() as usize;
        let grid_y = (sample.position.y / self.cell_size).floor() as usize;
        self.grid[grid_x][grid_y].push(index);
    }

    fn generate_initial_sample(&self) -> Sample {
        let x = rand::random::<f32>() * self.width;
        let y = rand::random::<f32>() * self.height;
        let dot_radius = rand::random_range(self.min_radius..=self.max_radius);
        Sample {
            position: Point { x, y },
            radius: dot_radius,
        }
    }

    fn generate_point_in_annulus(&self, sample: &Sample) -> Sample {
        let dot_radius = rand::random_range(self.min_radius..=self.max_radius);
        let minimum_distance =
            required_distance(sample.radius, dot_radius, self.spacing_multiplier);
        let maximum_distance = 2.0 * minimum_distance;
        let distance = rand::random_range(minimum_distance..=maximum_distance);
        let angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
        let x = sample.position.x + distance * angle.cos();
        let y = sample.position.y + distance * angle.sin();
        Sample {
            position: Point { x, y },
            radius: dot_radius,
        }
    }

    fn is_in_bounds(&self, sample: &Sample) -> bool {
        sample.position.x >= 0.0
            && sample.position.x < self.width
            && sample.position.y >= 0.0
            && sample.position.y < self.height
    }

    fn is_valid_sample(&self, sample: &Sample) -> bool {
        if !self.is_in_bounds(sample) {
            return false;
        }

        let nearby_samples = self.get_nearby_samples(sample);
        for neighbor in nearby_samples {
            let minimum_distance =
                required_distance(sample.radius, neighbor.radius, self.spacing_multiplier);
            if Self::distance(sample, neighbor) < minimum_distance {
                return false;
            }
        }

        true
    }

    fn get_nearby_samples(&self, sample: &Sample) -> Vec<&Sample> {
        let mut nearby_samples = Vec::new();
        let grid_x = (sample.position.x / self.cell_size).floor() as isize;
        let grid_y = (sample.position.y / self.cell_size).floor() as isize;

        for i in -self.grid_search_range..=self.grid_search_range {
            for j in -self.grid_search_range..=self.grid_search_range {
                let neighbor_x = grid_x + i;
                let neighbor_y = grid_y + j;

                if neighbor_x >= 0
                    && neighbor_y >= 0
                    && (neighbor_x as usize) < self.grid.len()
                    && (neighbor_y as usize) < self.grid[0].len()
                {
                    for &sample_index in &self.grid[neighbor_x as usize][neighbor_y as usize] {
                        nearby_samples.push(&self.sample_list[sample_index as usize]);
                    }
                }
            }
        }

        nearby_samples
    }

    fn distance(a: &Sample, b: &Sample) -> f32 {
        ((a.position.x - b.position.x).powi(2) + (a.position.y - b.position.y).powi(2)).sqrt()
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
        let mut sampler = PoissonSampler::new(100.0, 100.0, 3, 10, 1.0, 30);
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

        let mut sampler = PoissonSampler::new(width, height, 3, 10, 1.0, 30);
        let samples = sampler.generate_samples();

        for (i, sample) in samples.iter().enumerate() {
            assert!(
                sample.position.x >= 0.0 && sample.position.x < width,
                "Sample {} x out of bounds",
                i
            );
            assert!(
                sample.position.y >= 0.0 && sample.position.y < height,
                "Sample {} y out of bounds",
                i
            );
        }
    }

    #[test]
    fn test_generates_at_least_one_sample() {
        let mut sampler = PoissonSampler::new(100.0, 100.0, 3, 10, 1.0, 30);
        let samples = sampler.generate_samples();
        assert!(!samples.is_empty(), "No samples generated");
    }

    #[test]
    fn test_annulus_generation() {
        let sampler = PoissonSampler::new(100.0, 100.0, 3, 10, 1.0, 30);
        let center = Sample {
            position: Point { x: 50.0, y: 50.0 },
            radius: 5,
        };

        for _ in 0..100 {
            let sample = sampler.generate_point_in_annulus(&center);
            let dist = PoissonSampler::distance(&center, &sample);
            let minimum_distance = center.radius as f32 + sample.radius as f32;
            assert_in_range(
                dist,
                minimum_distance,
                2.0 * minimum_distance,
                "Annulus distance",
            );
        }
    }
}
