# Z- Mean Adjuster

A Rust implementation of a mean value adjuster. This class can adjust the mean (average) value of a dataset by weighing each value based on its z-score. Values closer to the mean are considered "better" and so are given more weight.

## Usage
Add the `z-mean-adjuster` to your project dependencies in `Cargo.toml`:


```toml
[dependencies]
z-mean-adjuster = "0.2.1"
```

Then use it in your code:

```rs
use mean_value_adjuster::Adjuster;

fn main() {
    let values = vec![100.0, 70.0, 88.0, 91.0, 85.0, 60.0, 99.0, 2.0];
    let adjuster = Adjuster::new(values);

    println!("True mean: {}", adjuster.get_true_mean());
    println!("Adjusted mean: {}", adjuster.get_adjusted_mean());
}
```
## Example

```rs
use mean_value_adjuster::Adjuster;

fn main() {
    let values = vec![100.0, 70.0, 88.0, 91.0, 85.0, 60.0, 99.0, 2.0];
    let adjuster = Adjuster::new(values);

    assert_eq!(adjuster.get_true_mean(), 74.375);
    assert_eq!(adjuster.get_adjusted_mean(), 83.54110999572508);
}
```

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.