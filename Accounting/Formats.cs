using System;
using System.Globalization;

static class AccountTypes
{
  public const string Assets = "Asset";
  public const string Liabilities = "Liability";
  public const string Income = "Income";
  public const string Expenses = "Expense";
  public const string Equity = "Equity";
}

static class DirectiveTypes
{
  public const string Open = "open";
  public const string Close = "close";
  public const string Note = "note";
  public static string Document = "document";
}

static class FormatEntries
{
  static string FormatCommodity(string commodity)
  {
    commodity = string.Join("_", commodity.Split(" "));
    return commodity.ToUpper();
  }

  static DateTime FormatDate(string date)
  {
    DateTime cDate;
    string[] dateFormat = new[] { "YYYY-MM-DD" };
    CultureInfo culture = new CultureInfo("en-US");

    if (DateTime.TryParse(date, out cDate))
    {
      return cDate.Date;
    }
    else
    {
      throw new FormatException($"{date} is not a proper date");
    }
  }


}
