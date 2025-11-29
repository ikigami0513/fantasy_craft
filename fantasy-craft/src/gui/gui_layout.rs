use std::collections::HashSet;

use hecs::Entity;
use macroquad::prelude::*;
use serde::Deserialize;
use crate::gui::gui_box::GuiBox;
use crate::gui::gui_draggable::GuiDraggable;
use crate::gui::gui_element::GuiElement;
use crate::gui::gui_local_offset::GuiLocalOffset;
use crate::gui::resources::UiResolvedRects;
use crate::prelude::{ComponentLoader, Context, Parent, Transform};
use crate::gui::gui_dimension::{GuiDimension, GuiDimensionLoaderData};

#[derive(Debug, Clone, Copy)]
pub struct GuiLayout {
    pub x: GuiDimension,
    pub y: GuiDimension
}

impl Default for GuiLayout {
    fn default() -> Self {
        Self {
            x: GuiDimension::Pixels(0.0),
            y: GuiDimension::Pixels(0.0)
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct GuiLayoutLoaderData {
    #[serde(default)]
    pub x: GuiDimensionLoaderData,
    #[serde(default)]
    pub y: GuiDimensionLoaderData,
}

pub struct GuiLayoutLoader;

impl ComponentLoader for GuiLayoutLoader {
    fn load(&self, ctx: &mut crate::prelude::Context, entity: hecs::Entity, data: &serde_json::Value) {
        let loader_data: GuiLayoutLoaderData = serde_json::from_value(data.clone())
            .unwrap_or_default();

        let parse_dimension = |loader_dim: GuiDimensionLoaderData| -> GuiDimension {
            match loader_dim {
                GuiDimensionLoaderData::Pixels(px) => GuiDimension::Pixels(px),
                GuiDimensionLoaderData::Percent(s) => {
                    let value = s.trim_end_matches('%')
                                 .parse::<f32>()
                                 .unwrap_or(0.0); // 0% par défaut
                    GuiDimension::Percent(value / 100.0) 
                }
            }
        };

        let component = GuiLayout {
            x: parse_dimension(loader_data.x),
            y: parse_dimension(loader_data.y),
        };

        ctx.world.insert_one(entity, component).expect("Failed to insert GuiLayout");
    }
}

pub fn gui_resolve_layout_system(ctx: &mut Context) {
    let (screen_w, screen_h) = (screen_width(), screen_height());
    
    ctx.resource_mut::<UiResolvedRects>().0.clear();

    // ... (Collecte des 'entities' ... c'est correct)
    let mut entities: HashSet<Entity> = ctx.world.query::<(&Parent, &GuiElement)>()
        .iter()
        .map(|(e, _)| e)
        .collect();

    entities.extend(
        ctx.world.query::<&GuiBox>().without::<&Parent>()
            .iter()
            .map(|(e, _)| e)
    );
    let mut entities_to_process: Vec<Entity> = entities.into_iter().collect();
    
    let mut iterations = 0;
    
    while !entities_to_process.is_empty() && iterations < 10 {
        // --- PHASE 1: COLLECTER LES MODIFICATIONS ---
        let mut results_to_apply = Vec::new();
        let mut processed_this_iteration = Vec::new();
        
        // Emprunt immuable de la map pour cette phase
        let resolved_rects_map = &ctx.resource::<UiResolvedRects>().0;

        entities_to_process.retain(|&entity| {
            let parent_opt = ctx.world.get::<&Parent>(entity).ok();

            let (parent_w, parent_h, parent_pos) = 
                if let Some(parent) = parent_opt.as_ref() { 
                    // Lecture depuis la map immuable
                    if let Some((pos, size)) = resolved_rects_map.get(&parent.0) {
                        (size.x, size.y, *pos)
                    } else {
                        return true; // Garder (parent pas encore traité)
                    }
                } else {
                    // ... (logique de la racine, inchangée)
                    if let Ok(layout) = ctx.world.get::<&GuiLayout>(entity) {
                        let root_x = layout.x.resolve(screen_w);
                        let root_y = layout.y.resolve(screen_h);
                        (screen_w, screen_h, vec2(root_x, root_y))
                    } else {
                        (screen_w, screen_h, Vec2::ZERO)
                    }
                };

            // 1. Résoudre Taille (lecture seule)
            let resolved_size = if let Ok(gui_box) = ctx.world.get::<&GuiBox>(entity) {
                vec2(gui_box.width.resolve(parent_w), gui_box.height.resolve(parent_h))
            } else {
                Vec2::ZERO
            };

            // 2. Résoudre Position (lecture seule)
            let mut resolved_pos;
            if parent_opt.is_some() {
                resolved_pos = parent_pos;
                if let Ok(local_offset) = ctx.world.get::<&GuiLocalOffset>(entity) {
                    resolved_pos.x += local_offset.x.resolve(parent_w);
                    resolved_pos.y += local_offset.y.resolve(parent_h);
                }
            } else {
                 if let Ok(layout) = ctx.world.get::<&GuiLayout>(entity) {
                    resolved_pos = vec2(layout.x.resolve(screen_w), layout.y.resolve(screen_h));
                } else if let Ok(transform) = ctx.world.get::<&Transform>(entity) {
                    resolved_pos = transform.position;
                } else {
                    resolved_pos = Vec2::ZERO;
                }
            }
            
            // 3. Stocker le résultat au lieu de l'insérer
            results_to_apply.push((entity, resolved_pos, resolved_size));
            
            processed_this_iteration.push(entity);
            false // Retirer de entities_to_process
        });

        // --- PHASE 2: APPLIQUER LES MODIFICATIONS ---
        
        // Failsafe
        if processed_this_iteration.is_empty() && !entities_to_process.is_empty() {
            eprintln!("Erreur de layout GUI : impossible de résoudre certaines entités.");
            break; 
        }

        let (_world, resources) = (&mut ctx.world, &mut ctx.resources);

        // 2. Obtenez l'accès mutable à la map DEPUIS 'resources'
        let rect_map_mut = &mut resources.get_mut::<UiResolvedRects>()
            .expect("Ressource UiResolvedRects manquante")
            .0;

        for (entity, pos, size) in results_to_apply {
            // 1. Insérer dans la map des ressources
            rect_map_mut.insert(entity, (pos, size));

            // 2. Mettre à jour le Transform
            let is_dragging = ctx.world.get::<&GuiDraggable>(entity)
                .map_or(false, |d| d.is_dragging);

            if !is_dragging {
                if let Ok(mut transform) = ctx.world.get::<&mut Transform>(entity) {
                    transform.position = pos;
                }
            }
        }
        
        iterations += 1;
    }
}
