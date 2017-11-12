/*
   Copyright 2017 Miguel Peláez <kernelfreeze@greenlab.games>
   Copyright 2017 Raúl Salas <raulsalas.martin@greenlab.games>
   
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use client::Client;
use scenes::scene::Scene;
use scenes::gui::Component;

use allegro::Color;

pub struct MainMenu;

impl Component for MainMenu {}

impl Scene for MainMenu {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(Color::from_rgb_f(0.8196, 0.8431, 1.0));

        None
    }
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {}
    }
}