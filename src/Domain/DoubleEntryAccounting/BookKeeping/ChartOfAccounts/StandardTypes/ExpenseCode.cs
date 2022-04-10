using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes
{
    public enum ExpenseCode
    {
        Unknown,
        /// <summary>
        /// In merchandising companies, cost of sales is normally the purchase price of the goods sold, including incidental costs. 
        /// In manufacturing businesses, it is the total production cost of the units sold. 
        /// Service companies do not have cost of sales.
        /// </summary>
        Cost_of_Sales,
        /// <summary>
        /// costs of promoting the business such as those incurred in newspaper publications, television and radio broadcasts, billboards, flyers, etc.
        /// </summary>
        Advertising,
        /// <summary>
        /// costs charged by banks for the use of their services
        /// </summary>
        Bank_Service_Charge,
        /// <summary>
        /// represents cost of gas, oil, courier fees, and other costs incurred by the business in transporting the goods sold to the customers.
        /// </summary>
        Delivery,
        /// <summary>
        /// refers to the portion of the cost of fixed assets (property, plant, and equipment) used for the operations of the period reported
        /// </summary>
        Depreciation,
        /// <summary>
        /// insurance premiums paid or payable to an insurance company who accepts to guarantee the business against losses from a specified even
        /// </summary>
        Insuranse,
        /// <summary>
        /// cost of borrowing money
        /// </summary>
        Interest,
        /// <summary>
        /// cost paid or to be paid to a lessor for the right to use a commercial property such as an office space, a storeroom, a building, etc.
        /// </summary>
        Rent,
        /// <summary>
        /// cost of repairing and servicing certain assets such as building facilities, machinery, and equipment
        /// </summary>
        Repairs_and_Maintenance,
        /// <summary>
        /// entertainment costs for customers, employees and owners. It is often coupled with traveling, 
        /// hence the account title Travel and Representation Expense
        /// </summary>
        Representation,
        /// <summary>
        /// compensation to employees for their services to the company
        /// </summary>
        Salaries,
        /// <summary>
        /// cost of supplies (ball pens, ink, paper, spare parts, etc.) used by the business. 
        /// Specific accounts may be in place such as Office Supplies Expense, Store Supplies Expense, and Service Supplies Expense.
        /// </summary>
        Supplies,
        /// <summary>
        /// business taxes, registration, and licensing fees paid to the government
        /// </summary>
        License_Fees_and_Taxes,
        /// <summary>
        /// cost of using communication and telephony technologies such as mobile phones, land lines, and internet
        /// </summary>
        Telecommunications,
        /// <summary>
        /// costs for the enhancement of employee skills
        /// </summary>
        Training_and_Development,
        /// <summary>
        ///  water and electricity costs paid or payable to utility companies
        /// </summary>
        Utilities
    }
}