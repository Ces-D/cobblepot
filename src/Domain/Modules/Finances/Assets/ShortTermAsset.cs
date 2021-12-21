namespace Cobblepot.Domain.Modules.Finances.Assets;

// see - https://www.investopedia.com/terms/c/currentassets.asp
public class ShortTermAsset : Entity, IAsset, IDescribable
{
    private string _title;
    private string _description;
    private ShortTermAssetType _type;
    private Money _value;
    private List<Note> _notes;

    public string Title { get { return _title; } set { _title = value; } }
    public Money Value { get { return _value; } }
    public string Description { get { return _description; } set { _description = value; } }
    public string AssetType { get { return _type.ToString(); } }

    public ShortTermAsset(Guid id, string title, string? description, ShortTermAssetType assetType, Money value) : base(id, SystemClock.Now)
    {
        _title = title;
        _description = description ?? "no description";
        _type = assetType;
        _value = value;
        _notes = new List<Note>();
    }

    public void AddNote(Note note) => _notes.Add(note);

    public override string ToString() => $"Short-Term Asset: {_title}, {_created}";
}

// TODO: add functions for updating these values