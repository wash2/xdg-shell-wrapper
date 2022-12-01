use sctk::{delegate_xdg_popup, delegate_xdg_shell, shell::xdg::popup::PopupHandler};

use crate::{shared_state::GlobalState, space::WrapperSpace};

impl<W: WrapperSpace> PopupHandler for GlobalState<W> {
    fn configure(
        &mut self,
        _conn: &sctk::reexports::client::Connection,
        _qh: &sctk::reexports::client::QueueHandle<Self>,
        popup: &sctk::shell::xdg::popup::Popup,
        config: sctk::shell::xdg::popup::PopupConfigure,
    ) {
        self.space.configure_popup(popup, config);
    }

    fn done(
        &mut self,
        _conn: &sctk::reexports::client::Connection,
        _qh: &sctk::reexports::client::QueueHandle<Self>,
        popup: &sctk::shell::xdg::popup::Popup,
    ) {
        self.space.close_popup(popup)
    }
}

delegate_xdg_shell!(@<W: WrapperSpace + 'static> GlobalState<W>);
delegate_xdg_popup!(@<W: WrapperSpace + 'static> GlobalState<W>);
