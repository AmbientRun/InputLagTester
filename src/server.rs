use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    entity::add_component,
    prelude::*,
};
use packages::this::{
    components::last_message,
    messages::{ClientToServer, ServerToClient},
};

#[main]
pub fn main() {
    ClientToServer::subscribe(|cx, msg| {
        ServerToClient {
            timestamp: msg.timestamp,
            index: msg.index,
        }
        .send_client_targeted_unreliable(cx.client_user_id().unwrap());
        add_component(
            cx.client_entity_id().unwrap(),
            last_message(),
            msg.timestamp,
        );
    });
}
