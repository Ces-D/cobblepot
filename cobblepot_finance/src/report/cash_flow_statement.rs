use crate::account::Account;

pub struct CashFlowStatement {
    /// Include any sources and uses of cash from activities.It reflects how much cash is generated from a products or services.
    operating_activities: Vec<Account>,
    /// Include any sources and uses of cash from a investments. Purchases or sales of assets, loans made to vendors or received from customers, or any payments related to mergers and acquisitions (M&A) are included in this category. In short, changes in equipment, assets, or investments relate to cash from investing.
    investing_activities: Vec<Account>,
}
