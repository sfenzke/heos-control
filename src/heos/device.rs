

//Basic model of a HEOS Capable Device. 
// Doesn't model all field currently becasue I don't need the right now.
pub struct Device {
    name: String,
    pid: i32,
    gid: Option<i32>,
}