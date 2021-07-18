export type AssetCategory =
  | "Cash"
  | "Account Receivable"
  | "Wage"
  | "Real Estate"
  | "Savings"
  | "Checking"
  | "Grant"
  | "Stock/Fund"
  | "CryptoCurrency"
  | "Bond"
  | "Gold"
  | "Miscellaneous";

export type LiabilityCategory =
  | "Loan"
  | "Rent"
  | "Mortgage"
  | "Home Expense"
  | "Insurance"
  | "Subscription"
  | "Leisure"
  | "Credit Balance"
  | "Miscellaneous";

class Calculator {
  /**
   * Useful when analyzing crypto currency value and comparing 
   */
  toDollars(){}

  /**
   * Take a certain kind of input and adds the values together
   */
  sum(){}

/**
 * Take a certain kind of input and subtract the values together
 */
  subtract(){}

  /**
   * Takes an object of keys sum and subtract and get the total. Useful for Total Assets 
   */
  total(){}


}

// TODO: start wit the data