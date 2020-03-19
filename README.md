# VISO: Visual 1Password terminal UI

# Install
`cargo build --release` will compile it to `target/release/viso`.

# Usage

`viso` makes heavy use of the `op` cli in order to function. You can install it
from [1password itself](https://support.1password.com/command-line-getting-started/).

After you have successfully set up `op` and have run `op signin` and have a valid
session token exported in your environment, you'll be able to run `viso`.


# Keybinds
- `j`: next item
- `k`: previous item
- `tab`: switch panes (vaults or items)
- `enter` (item pane only): view password
- `y`: (password visible screen) copy password with `wl-copy`.
