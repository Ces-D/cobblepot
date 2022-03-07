using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Interfaces;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes
{
    // see - https://www.principlesofaccounting.com/account-types/#:~:text=Account%20Types%20%20%20%20Account%20%20,%20%20Decrease%20%2036%20more%20rows%20
    public record AssetSubAccountType : ISubAccountType
    {
        public static readonly string AccountsReceivable = "Accounts Receivable";
        public static readonly string Building = "Building";
        /// <summary>
        /// Cash includes currency, coins, checking account balances, petty cash funds, and customers' checks that have not yet been deposited. 
        /// </summary>
        public static readonly string Cash = "Cash";
        public static readonly string PettyCash = "PettyCash";
        public static readonly string AvailableForSaleSecurities = "Available for Sale Securities";
        public static readonly string DomainName = "Domain Name";
        public static readonly string Equipment = "Equipment";
        public static readonly string InterestReceivable = "Interest Receivable";
        /// <summary>
        /// Investment includes annuities, bonds, the cash value of life insurance policies, mutual funds, pensions, retirement plans, (IRA, 401(k), 403(b), etc.) stocks
        /// </summary>
        public static readonly string Investments = "Investments";
        public static readonly string Inventory = "Inventory";
        public static readonly string Land = "Land";
        /// <summary>
        /// Personal property includes boats, collectibles, household furnishings, jewelry, vehicles
        /// </summary>
        public static readonly string PersonalProperty = "Personal Property";
        public static readonly string NotesReceivable = "Notes Receivable";
        public static readonly string Patent = "Patent";
        public static readonly string Copyright = "Copyright";
        public static readonly string License = "License";
        public static readonly string PrePaidInsurance = "PrePaid Insurance";
        public static readonly string PrePaidRent = "PrePaid Rent";
        public static readonly string Supplies = "Supplies";
        public static readonly string TradingSecurities = "Trading Securities";

    }
}
