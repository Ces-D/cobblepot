using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes
{
    public enum RevenueCode
    {
        Unknown,
        /// <summary>
        /// revenue earned from rendering services. 
        /// Other account titles may be used depending on the industry of the business, 
        /// such as Professional Fees for professional practice and Tuition Fees for schools.
        /// </summary>
        Service,
        /// <summary>
        /// revenue from selling goods to customers.
        /// </summary>
        Sales,
        /// <summary>
        /// earned from leasing out commercial spaces such as office space, stalls, booths, apartments, condominiums, etc.
        /// </summary>
        Rent,
        /// <summary>
        ///  revenue earned from lending money
        /// </summary>
        Interest_Income,
        /// <summary>
        /// from investment in associates, also dividend income arising from equity shares
        /// </summary>
        Investment_Income,
        /// <summary>
        /// earned by brokers and sales agents
        /// </summary>
        Commision_Income,
        /// <summary>
        /// earned by the owner of a property, patent, or copyrighted work for allowing others to use such in generating revenue
        /// </summary>
        Royalty_Income,
        /// <summary>
        /// earned by a franchisor in a franchise agreement
        /// </summary>
        Franchise_Fee,
        /// <summary>
        /// earned by professionals in rendering services that require their expertise
        /// </summary>
        Professional_Fees,
        /// <summary>
        /// when an asset is sold but not in the normal course of business, such as selling a used air-conditioning unit by a law firm
        /// </summary>
        Gain_on_Sale_of_Asset,
        Other,
    }
}
