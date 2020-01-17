use crate::sector::Sector;
use crate::player::Player;
use crate::utils::vector::Vertex;

pub fn decode_vertexes(array: &serde_json::Value) -> Vec<Vertex> {
    let mut vertexes = vec![];
    let decomposed = array.as_array().unwrap();
    for vec in decomposed {
        let v = vec.as_array().unwrap();
        let x = v[0].as_f64().unwrap();
        let y = v[1].as_f64().unwrap();
        vertexes.push(Vertex(x, y));
    }
    vertexes
}

pub fn decode_sectors(array: &serde_json::Value) -> Vec<Sector> {
    let mut sectors = vec![];
    let decomposed = array.as_array().unwrap();
    for sec in decomposed {
        let floor = sec["floor"].as_f64().unwrap();
        let ceil = sec["ceil"].as_f64().unwrap();
        let mut vertexes_id = vec![];
        for id in sec["vertexes_id"].as_array().unwrap() {
            vertexes_id.push(id.as_u64().unwrap());
        }
        let mut neighbors_id = vec![];
        for id in sec["neighbors_id"].as_array().unwrap() {
            vertexes_id.push(id.as_u64().unwrap());
        }
        sectors.push(
            Sector {
                floor: floor,
                ceil: ceil,
                vertexes_id: vertexes_id,
                neighbors_id: neighbors_id
            }
        )
    }
    sectors
}

pub fn decode_player(object: &serde_json::Value) -> Player {
    let mut player = Player::default();
    let decomposed = object.as_object().unwrap();
    let position = decomposed["position"].as_array().unwrap();
    player.position.x = position[0].as_f64().unwrap();
    player.position.y = position[1].as_f64().unwrap();
    player.position.z = position[2].as_f64().unwrap();
    let velocity = decomposed["velocity"].as_array().unwrap();
    player.velocity.x = velocity[0].as_f64().unwrap();
    player.velocity.y = velocity[1].as_f64().unwrap();
    player.angle = decomposed["angle"].as_f64().unwrap();
    player.sector = decomposed["angle"].as_u64().unwrap();
    player
}
