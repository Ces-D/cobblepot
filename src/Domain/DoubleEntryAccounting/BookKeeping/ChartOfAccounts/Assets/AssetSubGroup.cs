using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Assets
{
    public enum AssetSubGroup
    {
        Unknown,

        /// <summary>
        /// current asset
        /// 
        /// Cash on Hand - consists of un-deposited collections
        /// Cash in Bank - made up of bank accounts that are unrestricted as to withdrawal
        /// Short-term cash funds such as Petty Cash Fund, Payroll Fund, Tax Fund, etc.
        /// Cash Equivalents are short-term investments with very near maturity dates making them assets that are "as good as cash".
        /// </summary>
        Cash_and_Cash_Equivalents,

        /// <summary>
        /// current asset
        /// 
        /// Trading Securities are investments in stocks that are held with the purpose of trading (speculative investments)
        /// </summary>
        Trading_Securities,

        /// <summary>
        /// current asset
        /// 
        /// Accounts Receivable - receivables from customers arising from rendering of services or sale of goods
        /// Notes Receivable - receivables from customers which are backed up by promissory notes
        /// Other receivables representing claims from other parties such as: Rent Receivable, Interest Receivable, Dividend Receivable, etc.
        /// Allowance for Bad Debts - a contra-asset account deducted from Accounts Receivable. It represents the estimated uncollectible amount of the receivable.
        /// </summary>
        Trade_and_Other_Receivables,

        /// <summary>
        /// current asset
        /// 
        /// Inventories are assets that are held for sale in the normal operations of the business. 
        /// A service business normally has no inventory account.
        /// </summary>
        Inventories,

        /// <summary>
        /// current assets
        /// 
        /// Prepayments consists of costs already paid but are yet to be used or incurred. 
        /// Common prepaid expense accounts include: Office Supplies, Service Supplies, Prepaid Rent, and Prepaid Insurance.
        /// </summary>
        PrePaid_Expenses,

        /// <summary>
        /// non-current assets
        /// 
        /// PPE includes tangible assets that are expected to be used for more than one year.
        /// PPE accounts include: Land, Building, Machinery, Service Equipment, Computer Equipment, 
        /// Delivery Equipment, Furniture and Fixtures, Leasehold Improvements, etc.
        /// 
        /// Take note that land that is not used by the business in its operations 
        /// but is rather held for appreciation is not part of PPE but of investments.
        /// </summary>
        Property_Plant_and_Equipment,

        /// <summary>
        /// non-current assets
        /// 
        /// Investment in Long-Term Bonds, Investment in Associate, Investment in Subsidiary, 
        /// Investment Property, Long-Term Funds; these are investments that are intended to be held for more than one year.
        /// </summary>
        Long_Term_Investments,

        /// <summary>
        /// non-current assets
        /// 
        /// An intangible has no physical form but from which benefits can be derived and 
        /// its cost can be measured reliably.
        /// Intangibles include Patent for inventions, Copyright for authorship, 
        /// compositions and other literary works, Trademark, Franchise, Lease Rights, and Goodwill.
        /// </summary>
        Intangibles,

        /// <summary>
        /// non-current assets
        /// 
        /// Assets which cannot be classified under the usual non-current asset categories
        /// Includes: Advances to Officers, Directors, and Employees not collectible 
        /// within one year, Cash in Closed Banks, and Abandoned or Idle Property
        /// </summary>
        Other_Non_Current_Assets,
    }


}
