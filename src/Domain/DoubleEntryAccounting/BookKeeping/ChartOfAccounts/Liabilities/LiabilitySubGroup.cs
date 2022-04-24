using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Liabilities
{

    public enum LiabilitySubGroup
    {
        Unknown,
        /// <summary>
        /// current liability
        /// 
        /// refers to indebtedness that arise from purchase of goods, materials, supplies or services and other transaction in the normal course of business operations
        /// </summary>
        Accounts_Payable,

        /// <summary>
        /// current liability
        /// 
        /// obligations that are evidenced by promissory notes that are to be paid within 1 year
        /// </summary>
        Notes_Payable,

        /// <summary>
        /// current liability
        /// 
        /// current income tax obligation of the company payable to the government
        /// </summary>
        Income_Tax_Payable,

        /// <summary>
        /// current liability
        /// 
        /// includes wage taxes withheld from employees that will be remitted to the 
        /// appropriate government agency. Separate accounts for Social Security Payable and Medicare Payable are also often used
        /// </summary>
        Withholding_Tax_Payable,

        /// <summary>
        /// current liability
        /// 
        /// expenses already incurred but not yet paid. Accrued expense accounts 
        /// include: Salaries Payable, Rent Payable, Utilities Payable, 
        /// Interest Payable, Telecommunications Payable, and other unpaid expenses
        /// </summary>
        Accrued_Expenses,

        /// <summary>
        /// current liability
        /// 
        /// represents advanced payments from customers which requires settlement through delivery of goods or services in the future
        /// </summary>
        Unearned_Revenues,

        /// <summary>
        /// current liability, 
        /// Any other short-term payable, i.e. any obligation that is to be paid within 1 year after the balance sheet date
        /// </summary>
        Other_Short_Term_Payable,

        /// <summary>
        /// non-current liability
        /// 
        /// obligations evidenced by promissory notes which are to be paid beyond 1 year; 
        /// also commonly referred to as Loans Payable
        /// </summary>
        Long_Term_Notes_Payable,

        /// <summary>
        /// non-current liability
        /// 
        /// liabilities supported by a formal promise to pay a specified sum of money 
        /// at a future date and pay periodic interests. A bond has a stated face value 
        /// which is usually the final amount to be paid. Bonds can be traded in bond markets
        /// </summary>
        Bonds_Payable,

        /// <summary>
        /// non-current liability
        /// 
        /// long-term obligation to a bank or other financial institution, secured by real properties of the business
        /// </summary>
        Mortgage_Payable,

        /// <summary>
        /// non-current liability
        /// 
        /// Any other long-term payable, i.e. any obligation that is to be paid beyond 1 year
        /// </summary>
        Other_Long_Term_Payable,
    }

}
