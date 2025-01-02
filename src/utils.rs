use glib::object::IsA;
use gtk::prelude::WidgetExt;
use sctk::{
    delegate_foreign_toplevel_list, delegate_output, delegate_registry,
    foreign_toplevel_list::{ForeignToplevelList, ForeignToplevelListHandler},
    output::{OutputHandler, OutputState},
    reexports::{
        client::{globals::registry_queue_init, Connection, QueueHandle},
        protocols::ext::foreign_toplevel_list::v1::client::ext_foreign_toplevel_handle_v1,
    },
    registry::{ProvidesRegistryState, RegistryState},
};

use crate::{output_info::OutputObject, window_info::WindowObject};

pub fn get_data() -> (Vec<OutputObject>, Vec<WindowObject>) {
    // We create a temporary state to connect to the wayland compositor
    // After we grab it we initiate foreign-toplevel-list and xdg-output and run a single roundtrip
    //
    // The results from the roundtrip after registering all the events will be returned.
    //
    // TODO: Live-updating information
    // TODO: Use my own compositor IPC in order to preview window positions in workspaces

    struct TemporaryState {
        registry_state: RegistryState,
        output_state: OutputState,
        foreign_toplevel_list: ForeignToplevelList,
    }

    impl OutputHandler for TemporaryState {
        fn output_state(&mut self) -> &mut OutputState {
            &mut self.output_state
        }

        fn new_output(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: sctk::reexports::client::protocol::wl_output::WlOutput,
        ) {
        }

        fn update_output(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: sctk::reexports::client::protocol::wl_output::WlOutput,
        ) {
        }

        fn output_destroyed(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: sctk::reexports::client::protocol::wl_output::WlOutput,
        ) {
        }
    }

    impl ForeignToplevelListHandler for TemporaryState {
        fn foreign_toplevel_list_state(
            &mut self,
        ) -> &mut sctk::foreign_toplevel_list::ForeignToplevelList {
            &mut self.foreign_toplevel_list
        }

        fn new_toplevel(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }

        fn update_toplevel(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }

        fn toplevel_closed(
            &mut self,
            _: &Connection,
            _: &QueueHandle<Self>,
            _: ext_foreign_toplevel_handle_v1::ExtForeignToplevelHandleV1,
        ) {
        }
    }

    impl ProvidesRegistryState for TemporaryState {
        fn registry(&mut self) -> &mut RegistryState {
            &mut self.registry_state
        }

        sctk::registry_handlers!(OutputState);
    }

    delegate_registry!(TemporaryState);
    delegate_output!(TemporaryState);
    delegate_foreign_toplevel_list!(TemporaryState);

    let conn = &Connection::connect_to_env().unwrap();
    let (globals, mut queue) = registry_queue_init::<TemporaryState>(conn).unwrap();
    let qh = queue.handle();

    let mut state = TemporaryState {
        registry_state: RegistryState::new(&globals),
        output_state: OutputState::new(&globals, &qh),
        foreign_toplevel_list: ForeignToplevelList::new(&globals, &qh),
    };

    queue.roundtrip(&mut state).unwrap();

    let windows = state
        .foreign_toplevel_list
        .toplevels()
        .iter()
        .filter_map(|handle| state.foreign_toplevel_list.info(handle))
        .map(|info| WindowObject::new(info.identifier, info.title, info.app_id))
        .collect();
    let outputs = state
        .output_state
        .outputs()
        .filter_map(|wl_output| state.output_state.info(&wl_output))
        .map(OutputObject::new)
        .collect();

    (outputs, windows)
}

pub fn forall_siblings<W: IsA<gtk::Widget>, F: Fn(&gtk::Widget)>(w: &W, f: F) {
    let mut next_sibling = w.next_sibling();
    while let Some(child) = next_sibling {
        f(&child);
        next_sibling = child.next_sibling();
    }

    let mut prev_sibling = w.prev_sibling();
    while let Some(child) = prev_sibling {
        f(&child);
        prev_sibling = child.prev_sibling();
    }
}

pub fn make_output_mode_string(obj: &OutputObject) -> String {
    format!("{}x{}@{}", obj.size_w(), obj.size_h(), obj.refresh())
}
