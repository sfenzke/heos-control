

//Basic model of a HEOS Capable Device. 
// Doesn't model all field currently becasue I don't need the right now.
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub pid: i64,
    pub gid: Option<i64>,
}

impl Device {
    pub fn new(name:String, pid:i64, gid:Option<i64>) -> Self {
        Self {
            name,
            pid,
            gid
        }
    }
}