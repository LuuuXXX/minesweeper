use bevy::prelude::*;
use bevy::log;

use crate::resources::board::Board;
use crate::components::uncover::Uncover;
use crate::components::bomb::Bomb;
use crate::components::bomb_neighber::BombNeighbor;

use super::event::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event: EventReader<TileTriggerEvent>
) {
    for event in tile_trigger_event.iter() {
        log::info!("event: {:?}", event);
        if let Some(entity) = board.tile_to_uncover(&event.0) {
            log::info!("entity: {:?}", entity);
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        commands
            .entity(entity)
            .despawn_recursive();
        
        let coords = match board.get_coords(&entity) {
            Some(coor) => *coor,
            None => {
                panic!("failed to find coordinates")
            },
        }; 

        let (bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        log::info!("query coodinates is {:?}", coords);
        match board.try_uncover_tile(&coords) {
            Some(e) => log::debug!("Uncovered tile {:?} (entity: {:?})", coords, e),
            None => log::debug!("Tried to uncover tile an already covered tile")
        }

        if bomb.is_some() {
            log::info!("Bomb !");
        } else if bomb_counter.is_none() {
            log::info!("adjacent coverd tiles {:?}", board.adjacent_covered_tiles(coords));
            for entity in board.adjacent_covered_tiles(coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
    
}