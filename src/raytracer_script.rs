pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

pub struct Colour {

}

//A triangle but simply represents an infinite plane
pub struct Plane {
    normal: Vec3<f32>
}

pub struct Sphere {

}

pub struct Triangle {

}

pub struct RaytracerScript {

}

pub trait RaytracerScriptMethods {
    fn init(&mut self);
    fn update(&mut self);
}

impl RaytracerScript {
    fn new() -> Self {
        RaytracerScript {} 
    }
}

impl RaytracerScriptMethods for RaytracerScript {
    fn init(&mut self) {

    }

    fn update(&mut self) {

    }
}
