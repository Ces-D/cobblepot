namespace Cobblepot.Domain.Common;
using System.Reflection;
public abstract class Entity
{
    public Guid Id { get; init; }
    private List<PropertyInfo>? _properties { get; set; }
    private List<FieldInfo>? _fields { get; set; }
    protected Entity(Guid id) { Id = id; }
    public override bool Equals(object? obj)
    {
        if (obj is null) return false;
        if (obj is not Entity other) return false;
        if (ReferenceEquals(this, other)) return true;
        return Id.Equals(other.Id);
    }

    public static bool operator ==(Entity a, Entity b)
    {
        if (a is null && b is null)
            return true;

        if (a is null || b is null)
            return false;

        return a.Equals(b);
    }

    public static bool operator !=(Entity a, Entity b)
    {
        return !(a == b);
    }

    protected void CheckRule(IBusinessRule rule)
    {
        if (rule.IsBroken())
        {
            throw new BusinessRuleValidationException(rule);
        }
    }

    public override int GetHashCode()
    {
        unchecked
        {
            int hash = 17;
            foreach (var prop in GetProperties())
            {
                var value = prop.GetValue(this, null);
                hash = HashValue(hash, value);
            }

            foreach (var field in GetFields())
            {
                var value = field.GetValue(this);
                hash = HashValue(hash, value);
            }

            return hash;
        }
    }
    private IEnumerable<PropertyInfo> GetProperties()
    {
        if (this._properties == null)
        {
            this._properties = GetType()
                .GetProperties(BindingFlags.Instance | BindingFlags.Public)
                .ToList();
        }

        return this._properties;
    }

    private IEnumerable<FieldInfo> GetFields()
    {
        if (this._fields == null)
        {
            this._fields = GetType().GetFields(BindingFlags.Instance | BindingFlags.Public | BindingFlags.NonPublic)
                .ToList();
        }

        return this._fields;
    }

    private int HashValue(int seed, object? value)
    {
        var currentHash = value?.GetHashCode() ?? 0;

        return (seed * 23) + currentHash;
    }
}