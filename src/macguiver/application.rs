use crate::data::settings::Settings;
use crate::macguiver::buffer::DrawBuffer;
use crate::main_inner::Flags;
use crate::platform::GoLEmPlatform;
use embedded_graphics::pixelcolor::PixelColor;
use sdl3::event::Event;
use std::sync::{Arc, Mutex};

pub struct EventLoopState {
    events: Vec<Event>,
}

impl EventLoopState {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn events(&mut self) -> impl Iterator<Item = Event> + '_ {
        self.events.iter().cloned()
    }
}

pub trait Application {
    type Color: PixelColor;
    type Platform: GoLEmPlatform;

    fn settings(&self) -> &Settings;

    fn run(&mut self, flags: Flags);

    fn main_buffer(&mut self) -> &mut DrawBuffer<Self::Color>;

    fn database(&self) -> Arc<Mutex<golem_db::Connection>>;
    fn platform(&self) -> &Self::Platform;
    fn platform_mut(&mut self) -> &mut Self::Platform;

    fn hide_toolbar(&mut self);
    fn show_toolbar(&mut self);

    fn event_loop<R>(
        &mut self,
        loop_fn: impl FnMut(&mut Self, &mut EventLoopState) -> Option<R>,
    ) -> R;
}
