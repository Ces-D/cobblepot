use clap::{builder::PossibleValue, ValueEnum};
use cobblepot_core::chart_of_accounts::AccountCategory;

// Can only impl traits to values written in this mod. Had to extend by composition of target
// AccountCategory
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChartAccountCategory {
    AccountCategory(AccountCategory),
}
impl ValueEnum for ChartAccountCategory {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ChartAccountCategory::AccountCategory(AccountCategory::Asset),
            ChartAccountCategory::AccountCategory(AccountCategory::Equity),
            ChartAccountCategory::AccountCategory(AccountCategory::Expense),
            ChartAccountCategory::AccountCategory(AccountCategory::Revenue),
            ChartAccountCategory::AccountCategory(AccountCategory::Liability),
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ChartAccountCategory::AccountCategory(AccountCategory::Asset) => {
                PossibleValue::new("Asset").help("An asset")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Equity) => {
                PossibleValue::new("Equity").help("An equity")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Expense) => {
                PossibleValue::new("Expense").help("An expense")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Revenue) => {
                PossibleValue::new("Revenue").help("A revenue")
            },
            ChartAccountCategory::AccountCategory(AccountCategory::Liability) => {
                PossibleValue::new("Liability").help("A liability")
            },
        })
    }
}

impl std::fmt::Display for ChartAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

impl std::str::FromStr for ChartAccountCategory {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}
