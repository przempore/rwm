# TODO:

- [ ] Find a way to list all available 'alt+tab' windows at current desktop.
    This might help:
    ```C#
    public bool IsAltTabWindow()
    {
        if (!Visible) return false;
        if (!HasWindowTitle()) return false;
        if (IsAppWindow()) return true;
        if (IsToolWindow()) return false;
        if (IsNoActivate()) return false;
        if (!IsOwnerOrOwnerNotVisible()) return false;
        if (HasITaskListDeletedProperty()) return false;
        if (IsCoreWindow()) return false;
        if (IsApplicationFrameWindow() && !HasAppropriateApplicationViewCloakType()) return false;

        return true;
    }
    ```
    [source](https://github.com/kvakulo/Switcheroo/blob/21b07dffe7e42e91cede9d8e06cdcb32b768f149/Core/AppWindow.cs#L122)


- [ ] with a first version organize all windows on desktop.
