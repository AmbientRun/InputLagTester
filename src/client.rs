use ambient_api::{
    core::messages::Frame,
    element::{use_ref_with, use_rerender_signal, use_state},
    prelude::*,
};
use packages::this::messages::{ClientToServer, ServerToClient};

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

#[element_component]
fn App(hooks: &mut Hooks) -> Element {
    let samples = use_ref_with(hooks, |_| Vec::new());
    let redraw = use_rerender_signal(hooks);

    ServerToClient::subscribe({
        to_owned!(samples);
        move |cx, msg| {
            let took = (epoch_time() - msg.timestamp).as_secs_f32() * 1000.;
            {
                let mut samples = samples.lock();
                samples.push(took);
                if samples.len() > 1000 {
                    samples.remove(0);
                }
            }
            redraw();
            // println!("{}ms", took * 1000.);
        }
    });
    let (avg, max, last, count) = {
        let mut samples = samples.lock();
        let avg = samples.iter().sum::<f32>() / samples.len() as f32;
        let max = *samples
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.);
        let last = samples.last().unwrap_or(&0.);
        let count = samples.len();
        (avg, max, *last, count)
    };

    FlowColumn::el([
        Text::el(format!("Samples: {}", count)),
        Text::el(format!("Last: {}ms", last)),
        Text::el(format!("Avg: {}ms", avg)),
        Text::el(format!("Max: {}ms", max)),
    ])
}
