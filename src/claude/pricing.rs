use super::usage::UsageTotals;
use std::collections::HashMap;

pub struct PriceTable;

struct ModelPrice {
    input: f64,
    output: f64,
    cacheRead: f64,
}

impl PriceTable {
    pub fn costUsd(usageByModel: &HashMap<String, UsageTotals>) -> f64 {
        usageByModel.iter().map(|(model, usage)| Self::costFor(model, usage)).sum()
    }

    fn costFor(model: &str, usage: &UsageTotals) -> f64 {
        let price = Self::priceFor(model);
        let million = 1_000_000.0;
        usage.input as f64 * price.input / million
            + usage.output as f64 * price.output / million
            + usage.cacheWrite as f64 * price.input * 1.25 / million
            + usage.cacheRead as f64 * price.cacheRead / million
    }

    fn priceFor(model: &str) -> ModelPrice {
        let id = model.to_lowercase();
        let table: [(&str, f64, f64, f64); 9] = [
            ("fable-5-2", 8.0, 40.0, 0.20),
            ("mythos-5-2", 8.0, 40.0, 0.20),
            ("fable", 10.0, 50.0, 1.0),
            ("mythos", 10.0, 50.0, 1.0),
            ("opus-5-5", 4.0, 20.0, 0.20),
            ("opus", 5.0, 25.0, 0.50),
            ("sonnet-5", 2.0, 10.0, 0.10),
            ("sonnet", 3.0, 15.0, 0.30),
            ("haiku-5", 0.10, 0.50, 0.01),
        ];
        table
            .iter()
            .find(|(pattern, ..)| id.contains(pattern))
            .map(|(_, input, output, cacheRead)| ModelPrice {
                input: *input,
                output: *output,
                cacheRead: *cacheRead,
            })
            .unwrap_or(ModelPrice { input: 1.0, output: 5.0, cacheRead: 0.10 })
    }
}
