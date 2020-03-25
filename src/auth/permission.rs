pub enum Permission {
    /// Gives all permissions and bypasses overrides
    Administrator,
    /// Modify settings
    Manage,
    /// Modify role permissions
    ManageRoles,
    /// Modify other users
    ManageUsers,
    /// View album or image
    View,
    /// Edit album or image
    Edit,
}
