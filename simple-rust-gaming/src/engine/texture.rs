
struct Texture{
    path: &'static str,
}

impl Texture{
    pub fn new(path: &'static str) -> Texture{
        Texture{
            path: path,
        }
    }
}