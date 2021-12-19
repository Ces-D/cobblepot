namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/a/asset.asp 
public interface IAsset
{
    public string Title();
    public string Description();
    public string ToString();
    public Money Value();
}
