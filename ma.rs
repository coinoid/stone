pub trait MA {
    fn new_price(&mut self, price: f32);

    fn get_average(&self) -> f32;

    fn get_period(&self) -> u16;
}

// Simple moving average.
#[derive(Debug)]
pub struct Sim<'a> {
    prices: &'a mut Vec<f32>,
    period: u16,
    average: f32,
}

// Exponential moving average.
#[derive(Debug)]
pub struct Exp<'a> {
    prices: &'a mut Vec<f32>,
    period: u16,
    average: f32,
    multiplier: f32,
}

impl<'a> Sim<'a> {
    pub fn new(prices: &mut Vec<f32>, period: u16) -> Sim {
        let average = prices.iter().fold(0.0, |acc, price_i| acc + price_i);
        Sim {
            prices,
            period,
            average,
        }
    }
}

impl<'a> MA for Sim<'a> {
    fn new_price(&mut self, price: f32){
        self.prices.remove(0);
        self.prices.push(price);
        let self.average = self.prices.iter().fold(0.0, |acc, price_i| acc + price_i);
    }

    fn get_period(&self) -> u16 {
        self.period
    }
}

impl<'a> Exp<'a> {
    pub fn new(prices: &mut Vec<f32>, period: u16) -> Exp {
        let multiplier: f32 = (2 / (period + 1)) as f32;
        Exp {
            prices,
            period,
            multiplier,
        }
    }
}

impl<'a> MA for Exp<'a> {
    fn new_price(&mut self, price: f32) -> f32 {
        self.prices.remove(0);
        self.prices.push(price);

    }
}

pub mod tests {
    use super::*;

    #[test]
    fn test_ma_sim_new() {
        let mut prices = vec![2.0, 2.0, 2.0];
        let mut sim_ma = Sim::new(&mut prices, 3);
        assert_eq!((2.0 + 2.0 + 2.0) / 3.0, sim_ma.new_price(2.0));
    }
}
