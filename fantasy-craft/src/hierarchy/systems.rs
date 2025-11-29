use std::collections::HashMap;

use crate::prelude::{Context, LocalOffset, LocalVisible, Parent, Transform, Visible};

pub fn hierarchy_transform_update_system(ctx: &mut Context) {
    let mut world_position = HashMap::new();
    for (entity, transform) in ctx.world.query::<&Transform>().iter() {
        world_position.insert(entity, transform.position);
    }

    for (_, (child_transform, parent, local_offset)) in ctx.world.query::<(&mut Transform, &Parent, &LocalOffset)>().iter() {
        if let Some(&parent_world_pos) = world_position.get(&parent.0) {
            child_transform.position = parent_world_pos + local_offset.0;
        }
    }
}

pub fn hierarchy_visible_update_system(ctx: &mut Context) {
    
    // --- PASSE 0.A: Initialisation de `Visible` ---
    // On cherche les entités qui ont `Parent` MAIS n'ont PAS `Visible`
    let mut entities_to_add_visible = Vec::new();
    for (entity, _) in ctx.world.query::<&Parent>().without::<&Visible>().iter() {
        entities_to_add_visible.push(entity);
    }
    
    // On ajoute le composant `Visible` par défaut (true)
    for entity in entities_to_add_visible {
        ctx.world.insert_one(entity, Visible(true))
            .expect("Failed to add Visible component");
    }

    // --- PASSE 0.B: Initialisation de `LocalVisible` ---
    // On cherche les entités qui ont `Parent` MAIS n'ont PAS `LocalVisible`
    let mut entities_to_add_local = Vec::new();
    for (entity, _) in ctx.world.query::<&Parent>().without::<&LocalVisible>().iter() {
        entities_to_add_local.push(entity);
    }
    
    // On ajoute le composant `LocalVisible` par défaut (true)
    for entity in entities_to_add_local {
        ctx.world.insert_one(entity, LocalVisible(true))
            .expect("Failed to add LocalVisible component");
    }

    // --- PASSE 1: Cacher l'état de visibilité de tous les parents ---
    // (Votre code original)
    let mut world_visibility = HashMap::new();
    for (entity, visible) in ctx.world.query::<&Visible>().iter() {
        world_visibility.insert(entity, visible.0);
    }

    for (_, (parent, local_visible, child_visible)) in ctx.world.query::<(&Parent, &LocalVisible, &mut Visible)>().iter() {
        
        // Obtenir la visibilité du parent (par défaut `true` s'il n'a pas de composant)
        let parent_is_visible = world_visibility.get(&parent.0).copied().unwrap_or(true);

        // Obtenir l'état local de l'enfant
        let child_local_is_visible = local_visible.0;
        
        // L'enfant est visible SI le parent l'est ET SI il l'est localement
        child_visible.0 = parent_is_visible && child_local_is_visible;
    }
}
