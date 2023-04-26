use rusty_money::iso::Currency;

/// GAAP (Generally Accepted Accounting Principles) is a set of accounting rules, standards, and principles that dictate how companies must prepare and present their financial statements.
struct GAAP {
    primary_currency: Currency,
}

impl GAAP {
    fn new(primary_currency: Currency) -> GAAP {
        GAAP { primary_currency }
    }
}

// THIS takes care of reviewing accounts in a chart of accounts
// TODO: implement function checks for each of these key principles, this requires deciding on the
// methods  to check and ensure these standards are met

// Some of the key principles of GAAP include:
//
// Consistency: Accounting methods and procedures should be consistent from one period to the next to ensure comparability.
//
// Relevance: Financial statements should provide relevant and meaningful information to stakeholders, including investors, creditors, and regulatory bodies.
//
// Reliability: Financial information should be reliable, accurate, and free from bias, errors, or misstatements.
//
// Completeness: Financial statements should provide a complete picture of the company's financial position, performance, and cash flows.
//
// Objectivity: Accounting should be based on objective evidence and not on subjective opinion or estimates.
//
// Materiality: Financial information should be material, meaning that it is significant enough to influence the decisions of stakeholders.
//
// Prudence: Accountants should exercise caution and prudence when preparing financial statements, especially when dealing with uncertain or complex issues.
//
// Consensus: GAAP is based on a consensus of accounting professionals, standard-setters, and regulatory bodies to ensure that financial information is reported consistently and transparently across different industries and regions.
