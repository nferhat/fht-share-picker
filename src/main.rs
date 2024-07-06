use iced::alignment::Horizontal;
use iced::widget::{column, container, row, text, vertical_space, Canvas};
use iced::Length;
use sctk::output::{OutputHandler, OutputState};
use sctk::reexports::client as wl_client;
use sctk::registry::{ProvidesRegistryState, RegistryState};
use smithay_client_toolkit as sctk;

const APP_ID: &str = "fht.desktop.ScreenShareSourcePicker";
const TEXT_SIZE: f32 = 14.0;

mod font;
mod output_grid;
mod theme;

fn main() -> iced::Result {
    let settings = iced::Settings {
        default_font: font::MONO,
        fonts: font::load(),
        default_text_size: TEXT_SIZE.into(),
        id: Some(APP_ID.to_string()),
        antialiasing: false,
    };

    iced::application("Select your desired output", App::update, App::view)
        .settings(settings)
        .run_with(App::new)
}

#[derive(Default)]
struct App {
    output_grid: output_grid::OutputGrid,
    program_name: &'static str,
}

#[derive(Debug)]
enum Message {
    OutputSelected(String),
}

impl App {
    fn new() -> Self {
        // We need to get an output list.
        // There's not alot of ways, just use sctk (this is from their list_outputs example)
        struct ListOutputs(RegistryState, OutputState);
        impl OutputHandler for ListOutputs {
            fn output_state(&mut self) -> &mut OutputState {
                &mut self.1
            }

            fn new_output(
                &mut self,
                _: &sctk::reexports::client::Connection,
                _: &sctk::reexports::client::QueueHandle<Self>,
                _: sctk::reexports::client::protocol::wl_output::WlOutput,
            ) {
            }

            fn update_output(
                &mut self,
                _: &sctk::reexports::client::Connection,
                _: &sctk::reexports::client::QueueHandle<Self>,
                _: sctk::reexports::client::protocol::wl_output::WlOutput,
            ) {
            }

            fn output_destroyed(
                &mut self,
                _: &sctk::reexports::client::Connection,
                _: &sctk::reexports::client::QueueHandle<Self>,
                _: sctk::reexports::client::protocol::wl_output::WlOutput,
            ) {
            }
        }
        impl ProvidesRegistryState for ListOutputs {
            fn registry(&mut self) -> &mut RegistryState {
                &mut self.0
            }

            sctk::registry_handlers!(sctk::output::OutputState);
        }
        sctk::delegate_registry!(ListOutputs);
        sctk::delegate_output!(ListOutputs);
        // Connect to the compositor and ask for wl_output global
        let conn =
            wl_client::Connection::connect_to_env().expect("Failed to connec to WAYLAND_DISPLAY");
        let (globals, mut eq) = wl_client::globals::registry_queue_init::<ListOutputs>(&conn)
            .expect("Failed to create event queue!");
        let registry_state = RegistryState::new(&globals);
        let output_state = OutputState::new(&globals, &eq.handle());
        let mut list_outputs = ListOutputs(registry_state, output_state);
        // Do a single event queue roundtrip to receive data from the compositor.
        eq.roundtrip(&mut list_outputs)
            .expect("Failed to do event queue round trip");

        // Now the wl_output global has all the output information for us, translate to our data
        let outputs = list_outputs
            .1
            .outputs()
            .filter_map(|wl_output| {
                let info = list_outputs.1.info(&wl_output)?;
                let current_mode = info
                    .modes
                    .iter()
                    .find(|mode| mode.current)
                    .unwrap_or_else(|| info.modes.first().unwrap())
                    .clone();
                Some(output_grid::OutputData {
                    name: info.name.unwrap_or(info.model),
                    refresh: current_mode.refresh_rate as f32 / 1000.0, // default in millihertz
                    rec: iced::Rectangle::<u32> {
                        x: info.location.0 as u32,
                        y: info.location.1 as u32,
                        width: current_mode.dimensions.0 as u32,
                        height: current_mode.dimensions.1 as u32,
                    }
                    .into(),
                })
            })
            .collect::<Vec<_>>();

        let program_name = std::env::args()
            .skip(1)
            .next()
            .unwrap_or_else(|| "<unknown-application>".to_string());
        let program_name: &'static str = Box::leak(program_name.into_boxed_str());
        let output_grid = output_grid::OutputGrid::new(outputs);

        App {
            output_grid,
            program_name,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        #[allow(irrefutable_let_patterns)]
        let Message::OutputSelected(name) = message else {
            return iced::Task::none()
        };

        eprintln!("[select-output]/{name}");
        // TODO: Optimally, we'd use iced::exit(), but it causes a segmentation fault.
        // See: https://github.com/iced-rs/iced/issues/2482
        std::process::exit(0)
    }

    fn view(&self) -> iced::Element<Message, theme::Theme> {
        // Instead of having a list of outputs, we render them visually inside a canvas, giving the
        // user a better idea of what they are about to pick
        let canvas: iced::Element<String, theme::Theme> = Canvas::new(&self.output_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let canvas = canvas.map(Message::OutputSelected);

        let content = column![]
            .push(
                container(
                    text("Please select your desired output")
                        .horizontal_alignment(Horizontal::Center),
                )
                .center_x(Length::Fill),
            )
            .push(
                container(row![
                    text(self.program_name)
                        .horizontal_alignment(Horizontal::Center)
                        .style(theme::text::accent),
                    text(" wants to start a screecast session")
                        .horizontal_alignment(Horizontal::Center),
                ])
                .center_x(Length::Fill),
            )
            .push(vertical_space().height(12))
            .push(
                container(canvas)
                    .style(theme::container::bordered)
                    .padding(10)
                    .height(Length::Shrink)
                    .center_x(Length::Fill),
            );

        container(content).padding(10).fill().into()
    }
}
