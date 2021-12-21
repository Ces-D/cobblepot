namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/a/asset.asp 
internal interface IAsset
{
    public Money Value { get; }
}