using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Cobblepot.Domain.DoubleEntryAccounting.BookKeeping.ChartOfAccounts.StandardTypes.AccountTypes
{
    
    public enum AssetCode : int
    {
        Unknown,
        AccountsReceivable,
        Building,
        Cash,
        PettyCash,
        AvailableForSaleSecurities,
        DomainName,
        Equipment,
        InterestReceivable,
        Investments,
        Inventory,
        Land,
        PersonalProperty,
        NotesReceivable,
        Patent,
        Copyright,
        License,
        PrePaidInsurance,
        PrePaidRent,
        Supplies,
        TradingSecurities

    }
}
