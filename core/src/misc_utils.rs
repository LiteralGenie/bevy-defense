use bevy::prelude::*;

pub fn find_materials(
    entity: &Entity,
    mat_query: &Query<&Handle<StandardMaterial>>,
    children_query: &Query<&Children>,
) -> Vec<(Entity, Handle<StandardMaterial>)> {
    let mut to_check = vec![entity];
    let mut results = vec![];

    while to_check.len() > 0 {
        let e = to_check.pop().unwrap();
        if let Ok(handle) = mat_query.get(*e) {
            results.push((*e, handle.clone()));
        }

        if let Ok(children) = children_query.get(*e) {
            to_check.extend(children);
        }
    }

    return results;
}
