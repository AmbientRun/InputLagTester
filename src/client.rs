use crate::packages::this::components::last_message;
use ambient_api::{
    core::{messages::Frame, player::components::is_player},
    element::{use_frame, use_query, use_ref_with, use_rerender_signal, use_spawn, use_state},
    entity::get_component,
    prelude::*,
};
use packages::this::messages::{ClientToServer, ServerToClient};
use parking_lot::Mutex;
use std::sync::Arc;

#[main]
pub fn main() {
    App.el().spawn_interactive();
    let mut index = 0;
    Frame::subscribe(move |_| {
        ClientToServer {
            timestamp: epoch_time(),
            index,
        }
        .send_server_unreliable();
        index += 1;
    });
}

#[derive(Debug, Clone)]
struct Samples {
    samples: Vec<f32>,
    total: u32,
    start_time: Duration,
}
impl Samples {
    fn new() -> Self {
        Self {
            samples: Vec::new(),
            total: 0,
            start_time: epoch_time(),
        }
    }
    fn add(&mut self, sample: f32) {
        self.samples.push(sample);
        if self.samples.len() > 10000 {
            self.samples.remove(0);
        }
        self.total += 1;
    }
}

#[element_component]
fn App(hooks: &mut Hooks) -> Element {
    let msg_samples = use_ref_with(hooks, |_| Samples::new());
    let component_samples = use_ref_with(hooks, |_| Samples::new());
    let redraw = use_rerender_signal(hooks);
    use_spawn(hooks, {
        to_owned!(msg_samples);
        move |_| {
            ServerToClient::subscribe({
                to_owned!(msg_samples);
                move |cx, msg| {
                    let took = (epoch_time() - msg.timestamp).as_secs_f32() * 1000.;
                    msg_samples.lock().add(took);
                }
            });
            |_| {}
        }
    });
    use_frame(hooks, {
        to_owned!(component_samples);
        move |_| {
            if let Some(v) = get_component(player::get_local(), last_message()) {
                let took = (epoch_time() - v).as_secs_f32() * 1000.;
                component_samples.lock().add(took);
            }
            redraw();
        }
    });
    FlowColumn::el([
        Text::el("Message samples:").header_style(),
        RenderSamples::el(msg_samples),
        Text::el("Component samples:").header_style(),
        RenderSamples::el(component_samples),
    ])
}

#[element_component]
fn RenderSamples(hooks: &mut Hooks, samples: Arc<Mutex<Samples>>) -> Element {
    let (avg, max, last, count, samples_per_second) = {
        let mut samples = samples.lock();
        let avg = samples.samples.iter().sum::<f32>() / samples.samples.len() as f32;
        let max = *samples
            .samples
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.);
        let last = samples.samples.last().unwrap_or(&0.);
        let samples_per_second =
            samples.total as f32 / (epoch_time() - samples.start_time).as_secs_f32();
        (avg, max, *last, samples.total, samples_per_second)
    };
    let n_players = use_query(hooks, is_player()).len();

    FlowColumn::el([
        Text::el(format!("Players: {}", n_players)),
        Text::el(format!("Samples: {}", count)),
        Text::el(format!("Samples per second: {}", samples_per_second)),
        Text::el(format!("Last: {}ms", last)),
        Text::el(format!("Avg: {}ms", avg)),
        Text::el(format!("Max: {}ms", max)),
    ])
}
