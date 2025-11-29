use hecs::Entity;

pub struct UiClickEvent {
    pub action_id: String,
    pub entity: Entity
}
