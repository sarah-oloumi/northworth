use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoneyAmount {
    pub cents: i64,
    pub currency: Currency,
}

impl MoneyAmount {
    pub fn cad_dollars(dollars: i64) -> Self {
        Self {
            cents: dollars * 100,
            currency: Currency::Cad,
        }
    }

    pub fn cad_cents(cents: i64) -> Self {
        Self {
            cents,
            currency: Currency::Cad,
        }
    }

    pub fn is_outflow(&self) -> bool {
        self.cents < 0
    }

    pub fn is_inflow(&self) -> bool {
        self.cents > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Cad,
    Usd,
    Other(String),
}
