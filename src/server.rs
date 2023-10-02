use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
};
use packages::this::messages::{ClientToServer, ServerToClient};

#[main]
pub fn main() {
    ClientToServer::subscribe(|cx, msg| {
        ServerToClient {
            timestamp: msg.timestamp,
            index: msg.index,
        }
        .send_client_targeted_unreliable(cx.client_user_id().unwrap());
    });
}
