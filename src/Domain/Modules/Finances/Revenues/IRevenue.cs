namespace Cobblepot.Domain.Modules.Finances.Revenues;

// see - https://www.investopedia.com/terms/r/revenue.asp
// see - https://www.investopedia.com/terms/c/cashaccounting.asp
// see - https://www.investopedia.com/terms/a/accrualaccounting.asp

internal interface IRevenue
{
    public Money Revenue { get; }
    public RevenueType RevenueType { get; }

    // Operating Revenue - sales from a company's core business
    // Non-operating Revenue - derived from secondary sources, unpredicatble, non recurring
    // 
}

// TODO: consider adding rules to revenues