use std::fmt;

use crate::{Close, Next, Reset, High, Low};

use super::{TrueRange, SimpleMovingAverage};

pub struct Supertrend {
  index: usize,
  multiplier: f64,
  previous_close: f64,
  previous_trend: i64,
  previous_up: f64,
  previous_down: f64,
  tr: TrueRange,
  tr_sma: SimpleMovingAverage
}

impl Supertrend {
  pub fn new(period: usize, multiplier: f64) -> Self {
      Self {
        index: 0,
        multiplier,
        previous_trend: 0,
        previous_up: 0.0,
        previous_down: 0.0,
        previous_close: 0.0,
        tr: TrueRange::new(),
        tr_sma: SimpleMovingAverage::new(period).unwrap()
      }
  }
}

impl<T: High + Low + Close> Next<&T> for Supertrend {
  type Output = (f64, f64, i64);

  fn next(&mut self, input: &T) -> Self::Output {
    // hl2
    let hl2 = (input.high() + input.low()) / 2.0;
    // tr and then sma the tr to act like an alternative to atr
    let tr = self.tr.next(input);
    let tr_sma = self.tr_sma.next(tr);
    // initial upper/lower bands + trend
    let initial_up = hl2 - (self.multiplier * tr_sma);
    let initial_down = hl2 + (self.multiplier * tr_sma);
    let initial_trend = 1;
    // handle first candle
    if self.index == 0 {
      self.previous_up = initial_up;
      self.previous_down = initial_down;
      self.previous_trend = initial_trend;
      self.previous_close = input.close();
      self.index += 1;
      return (initial_up, initial_down, initial_trend);
    }
    // calculate final up/down/trend
    let final_up = if self.previous_close > self.previous_up {
      if initial_up > self.previous_up {
        initial_up
      } else {
        self.previous_up
      }
    } else {
      initial_up
    };
    let final_down = if self.previous_close < self.previous_down {
      if self.previous_down < initial_down {
        self.previous_down
      } else {
        initial_down
      }
    } else {
      initial_down
    };
    let final_trend = if self.previous_trend == -1 && input.close() > self.previous_down {
      1
    } else if self.previous_trend == 1 && input.close() < self.previous_up {
      -1
    } else {
      self.previous_trend
    };
    self.previous_up = final_up;
    self.previous_down = final_down;
    self.previous_trend = final_trend;
    self.previous_close = input.close();
    self.index += 1;
    return (final_up, final_down, final_trend);
  }
}

impl Default for Supertrend {
  fn default() -> Self {
      Self::new(10, 3.0)
  }
}

impl fmt::Display for Supertrend {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Supertrend")
  }
}

impl Reset for Supertrend {
  fn reset(&mut self) {
    self.index = 0;
    self.previous_trend = 0;
    self.previous_up = 0.0;
    self.previous_down = 0.0;
    self.previous_close = 0.0;
    self.tr.reset();
    self.tr_sma.reset();
  }
}
