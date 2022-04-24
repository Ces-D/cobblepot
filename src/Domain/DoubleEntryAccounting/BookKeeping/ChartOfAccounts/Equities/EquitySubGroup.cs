using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.Equities
{


    public enum EquitySubGroup
    {
        Unknown,

        /// <summary>
        /// The term used for equity depends upon the form of business organization.
        ///
        /// For sole proprietorships, it is known as owner's equity.
        /// For partnerships, it is called partners' equity.
        /// For corporations, specifically stock corporations, it is known as stockholders' equity.
        /// </summary>
        Stockholders,

        /// <summary>
        /// 1. Common Stock - also known as Ordinary Shares. It represents ownership in a corporation. 
        /// Common stockholders are given rights to receive dividends and voting rights in electing a board of directors.
        /// 2. Preferred Stock - also known as Preference Shares. Preferred stockholders enjoy fixed dividend rates and are paid first before the common stockholders. Preferred stocks normally do not possess voting rights, unless stated.
        /// 3. Subscribed Capital Stock - common or preferred stocks subscribed but not yet paid, hence not yet issued
        /// </summary>
        Capital,

        /// <summary>
        /// 1. Additional Paid-in Capital - also known as Share Premium; contribution from stockholders in excess of the par or stated value of the stocks issued
        /// 2. Unrealized Gains and Losses - gains and losses that cannot be included in the income statement as per accounting standards, such as Unrealized Gain/Loss on Financial Assets and Unrealized Gain/Loss on Translation Adjustment
        /// 3. Revaluation Surplus - increase in the value of a fixed asset after appropriate appraisal
        /// 4. Appropriated Retained Earnings - company's earnings set aside for a specific purpose (such as Retained Earnings Appropriated for Plant Expansion), hence cannot be distributed as dividends to the stockholders
        /// </summary>
        Reserves,

        /// <summary>
        /// Treasury stocks are shares of the corporation that have been issued and then were reacquired but not canceled. 
        /// </summary>
        Treasury,

    }

}


// TODO: review if this is the best way to categorize equity