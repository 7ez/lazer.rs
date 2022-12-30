use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct UserGrades {
    pub ssh:     u32, // Silver SS
    pub ss:      u32, // Normal SS
    pub sh:      u32, // Silver S
    pub s:       u32, // Normal S
    pub a:       u32, // A
    
}