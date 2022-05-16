using System;

namespace Cobblepot.BuildingBlocks.Domain;

public abstract class Aggregate
{
    public long Version { get; private set; }
    public bool IsDeleted { get; private set; }
}