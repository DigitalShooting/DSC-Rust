


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub title: String,
    pub rings: Vec<Ring>,
    pub rings_draw_only: Vec<RingDrawOnly>,
    pub default_hit_color: WebColor,
    pub default_zoom: Zoom,
    pub min_zoom: Zoom,
    pub inner_ten: i32, // renamed innenZehner
    pub trial_corner_color: WebColor, // renamed probeEcke.color probeEcke.alpha
    pub bullet_diameter: f64, // renamed kugelDurchmesser
}

pub type Zoom = f32;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ring {
    pub value: i32,
    pub width:  f64,
    pub color: WebColor,
    pub has_text: bool, // renamed text
    pub text_color: WebColor,
    pub zoom: Zoom,
    pub hit_color: WebColor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RingDrawOnly {
    pub width:  f64,
    pub color: WebColor,
    pub has_text: bool,
    pub text_color: WebColor,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebColor {
    pub hex: String,
    pub alpha: f32,
}









// use session::series::Draw;
// use simplesvg::*;
//
// impl Draw for Target {
//     fn draw(&self) -> Vec<Fig> {
//         let mut figs: Vec<Fig> = vec![];
//         for ring in &self.rings {
//             figs.extend(ring.draw());
//         }
//         figs
//     }
// }
//
//
//
// impl Ring {
//     fn draw(&self) -> Vec<Fig> {
//         let circle = Fig::Circle(250., 250., self.width as f32);
//         let attr = Attr::default().stroke(Color(0, 0, 0));
//         vec![Fig::Styled(attr, Box::new(circle))]
//     }
// }
