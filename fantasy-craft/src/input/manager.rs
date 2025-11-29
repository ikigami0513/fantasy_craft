use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    Key(KeyCode),
    Mouse(MouseButton)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardLayout {
    Qwerty,
    Azerty,
}

pub struct InputManager {
    bindings: HashMap<String, Vec<InputVariant>>,
    /// Layout is used to adapt key interpretation on Native targets.
    /// On Web, we ignore this because browsers send physical scancodes.
    pub layout: KeyboardLayout,
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            layout: KeyboardLayout::Qwerty, // Default standard
        }
    }
}

impl InputManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_layout(&mut self, layout: KeyboardLayout) {
        self.layout = layout;
        // Ideally, we should reload bindings here if they were already loaded,
        // but for simplicity, we assume set_layout is called before loading config.
        info!("InputManager: Keyboard layout set to {:?}", layout);
    }

    pub fn bind(&mut self, action: &str, input: InputVariant) {
        self.bindings
            .entry(action.to_string())
            .or_insert_with(Vec::new)
            .push(input);
    }

    /// Loads bindings from a JSON string content.
    pub fn load_from_string(&mut self, json_content: &str) {
        let json_bindings: HashMap<String, Vec<String>> = match serde_json::from_str(json_content) {
            Ok(data) => data,
            Err(e) => {
                error!("InputManager: Failed to parse bindings JSON. Error: {}", e);
                return;
            }
        };

        // Clear existing bindings to avoid duplicates if reloading
        self.bindings.clear();

        for (action, inputs) in json_bindings {
            for input_str in inputs {
                let variants = self.parse_input_string(&input_str);
                
                if variants.is_empty() {
                    warn!("InputManager: Unknown input key '{}' for action '{}'", input_str, action);
                }

                for variant in variants {
                    self.bind(&action, variant);
                }
            }
        }
        info!("InputManager: Bindings loaded.");
    }

    pub fn is_action_down(&self, action: &str) -> bool {
        if let Some(inputs) = self.bindings.get(action) {
            for input in inputs {
                match input {
                    InputVariant::Key(k) => if is_key_down(*k) { return true; },
                    InputVariant::Mouse(b) => if is_mouse_button_down(*b) { return true; }
                }
            }
        }
        false
    }

    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        if let Some(inputs) = self.bindings.get(action) {
            for input in inputs {
                match input {
                    InputVariant::Key(k) => if is_key_pressed(*k) { return true; },
                    InputVariant::Mouse(b) => if is_mouse_button_pressed(*b) { return true; }
                }
            }
        }
        false
    }

    /// Converts a string representation of a key to a LIST of Macroquad InputVariants.
    fn parse_input_string(&self, s: &str) -> Vec<InputVariant> {
        let mut variants = Vec::new();

        // Helper to check if we are in a context where remapping is needed
        // On WASM, we trust physical keys (Browser KeyCodes).
        // On Native, we respect the user's Layout setting.
        let use_azerty_mapping = {
            #[cfg(target_arch = "wasm32")]
            { false }
            #[cfg(not(target_arch = "wasm32"))]
            { self.layout == KeyboardLayout::Azerty }
        };

        match s {
            // Mouse
            "MouseLeft" => variants.push(InputVariant::Mouse(MouseButton::Left)),
            "MouseRight" => variants.push(InputVariant::Mouse(MouseButton::Right)),
            "MouseMiddle" => variants.push(InputVariant::Mouse(MouseButton::Middle)),
            
            // --- MOVEMENT KEYS REMAPPING ---
            // The JSON config should use standard QWERTY names ("W", "A", "S", "D")
            
            "W" | "ScanCodeW" => {
                if use_azerty_mapping {
                    variants.push(InputVariant::Key(KeyCode::Z)); // Native AZERTY
                } else {
                    variants.push(InputVariant::Key(KeyCode::W)); // Native QWERTY or Web
                }
            },
            
            "A" | "ScanCodeA" => {
                if use_azerty_mapping {
                    variants.push(InputVariant::Key(KeyCode::Q));
                } else {
                    variants.push(InputVariant::Key(KeyCode::A));
                }
            },

            "Z" | "ScanCodeZ" => {
                if use_azerty_mapping {
                    variants.push(InputVariant::Key(KeyCode::W));
                } else {
                    variants.push(InputVariant::Key(KeyCode::Z));
                }
            },

            "Q" | "ScanCodeQ" => {
                if use_azerty_mapping {
                    variants.push(InputVariant::Key(KeyCode::A));
                } else {
                    variants.push(InputVariant::Key(KeyCode::Q));
                }
            },
            
            "M" => {
                if use_azerty_mapping {
                     // M is tricky on AZERTY (it's to the right of L, often semicolon on US)
                     // But usually M on AZERTY maps to KeyCode::Semicolon in some raw modes
                     // or KeyCode::M. For simplicity, we assume M is M logic here or comma.
                     // This often needs specific tweaking per OS.
                     variants.push(InputVariant::Key(KeyCode::M));
                } else {
                     variants.push(InputVariant::Key(KeyCode::M));
                }
            },

            // --- INVARIANTS ---
            "S" | "ScanCodeS" => variants.push(InputVariant::Key(KeyCode::S)),
            "D" | "ScanCodeD" => variants.push(InputVariant::Key(KeyCode::D)),
            
            "Space" => variants.push(InputVariant::Key(KeyCode::Space)),
            "Escape" => variants.push(InputVariant::Key(KeyCode::Escape)),
            "Enter" => variants.push(InputVariant::Key(KeyCode::Enter)),
            "Tab" => variants.push(InputVariant::Key(KeyCode::Tab)),
            "LeftShift" => variants.push(InputVariant::Key(KeyCode::LeftShift)),
            "RightShift" => variants.push(InputVariant::Key(KeyCode::RightShift)),
            "LeftControl" => variants.push(InputVariant::Key(KeyCode::LeftControl)),
            "RightControl" => variants.push(InputVariant::Key(KeyCode::RightControl)),
            
            "Up" => variants.push(InputVariant::Key(KeyCode::Up)),
            "Down" => variants.push(InputVariant::Key(KeyCode::Down)),
            "Left" => variants.push(InputVariant::Key(KeyCode::Left)),
            "Right" => variants.push(InputVariant::Key(KeyCode::Right)),
            
            // Letters (simplified)
            "B" => variants.push(InputVariant::Key(KeyCode::B)),
            "C" => variants.push(InputVariant::Key(KeyCode::C)),
            "E" => variants.push(InputVariant::Key(KeyCode::E)),
            "F" => variants.push(InputVariant::Key(KeyCode::F)),
            "G" => variants.push(InputVariant::Key(KeyCode::G)),
            "H" => variants.push(InputVariant::Key(KeyCode::H)),
            "I" => variants.push(InputVariant::Key(KeyCode::I)),
            "J" => variants.push(InputVariant::Key(KeyCode::J)),
            "K" => variants.push(InputVariant::Key(KeyCode::K)),
            "L" => variants.push(InputVariant::Key(KeyCode::L)),
            "N" => variants.push(InputVariant::Key(KeyCode::N)),
            "O" => variants.push(InputVariant::Key(KeyCode::O)),
            "P" => variants.push(InputVariant::Key(KeyCode::P)),
            "R" => variants.push(InputVariant::Key(KeyCode::R)),
            "T" => variants.push(InputVariant::Key(KeyCode::T)),
            "U" => variants.push(InputVariant::Key(KeyCode::U)),
            "V" => variants.push(InputVariant::Key(KeyCode::V)),
            "X" => variants.push(InputVariant::Key(KeyCode::X)),
            "Y" => variants.push(InputVariant::Key(KeyCode::Y)),

            "0" => variants.push(InputVariant::Key(KeyCode::Key0)),
            "1" => variants.push(InputVariant::Key(KeyCode::Key1)),
            "2" => variants.push(InputVariant::Key(KeyCode::Key2)),
            "3" => variants.push(InputVariant::Key(KeyCode::Key3)),
            "4" => variants.push(InputVariant::Key(KeyCode::Key4)),
            "5" => variants.push(InputVariant::Key(KeyCode::Key5)),
            "6" => variants.push(InputVariant::Key(KeyCode::Key6)),
            "7" => variants.push(InputVariant::Key(KeyCode::Key7)),
            "8" => variants.push(InputVariant::Key(KeyCode::Key8)),
            "9" => variants.push(InputVariant::Key(KeyCode::Key9)),

            _ => {},
        };
        
        variants
    }
}
