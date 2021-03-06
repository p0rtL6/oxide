use serenity::{
    client::Context,
    model::{channel::Message, interactions::message_component},
};

mod commands;
mod components;
mod requests;

pub async fn handle_request(ctx: Context, msg: Message) {
    requests::handle_request(ctx, msg).await;
}

pub fn handle_commands(ctx: Context, msg: Message) {
    commands::handle_commands(ctx, msg);
}

pub fn handle_component_interaction(
    ctx: Context,
    component_interaction: message_component::MessageComponentInteraction,
) {
    components::handle_component_interaction(ctx, component_interaction);
}
