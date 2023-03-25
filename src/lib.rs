use std::f64::consts::PI;
use std::vec::Vec;

struct Adjuster {
    votes: Vec<f64>,
}

impl Adjuster {
    fn new(ls: Vec<f64>) -> Adjuster {
        Adjuster { votes: ls }
    }

    fn apply_weight(&self) -> Vec<(f64, f64)> {
        let mn = self.get_true_mean();
        let sd = self.get_standard_deviation();
        let mut weights = Vec::new();

        for x in &self.votes {
            let z_score = if sd == 0.0 { 0.0 } else { (x - mn) / sd };
            let weight = (PI / 2.0 * z_score).cos().max(0.0);
            weights.push((*x, weight));
        }

        weights
    }

    fn get_true_mean(&self) -> f64 {
        self.votes.iter().sum::<f64>() / self.votes.len() as f64
    }

    fn get_standard_deviation(&self) -> f64 {
        let mean = self.get_true_mean();
        let variance = self.votes.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.votes.len() as f64;
        variance.sqrt()
    }

    fn get_adjusted_mean(&self) -> f64 {
        let weights = self.apply_weight();
        let (sum_value, sum_weight) = weights.iter().fold((0.0, 0.0), |acc, (value, weight)| (acc.0 + value * weight, acc.1 + weight));
        sum_value / sum_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjusted_mean() {
        let values = vec![100.0, 70.0, 88.0, 91.0, 85.0, 60.0, 99.0, 2.0];
        let adjuster = Adjuster::new(values);
        assert_eq!(adjuster.get_true_mean(), 74.375);
        assert_eq!(adjuster.get_adjusted_mean(), 83.54110999572508);
    }
}

fn main() {
    // run tests
    tests::test_adjusted_mean();
}
