
use discipline::*;



pub fn lg_discipline() -> Discipline {
    Discipline {
        id: String::from("demoPart"),
        title: String::from("LG Demo"),
        interface: Interface::Demo {interval: 1000, max_shots: Some(40)},
        // interface: Interface::ESA {
        //     port: "/dev/ttyS0".to_string(),
        //     on_part_band: 3,
        //     on_shot_band: 2,
        // },
        time: Time::FirstShot { duration: 40 },
        target: Target {
            title: String::from("LG 10m"),
            rings: vec![
                Ring {
                    value: 10,
                    width: 0.5,
                    color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    has_text: false,
                    text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                    zoom: 4.0,
                    hit_color: WebColor { hex: String::from("red"), alpha: 1.0 },
                },
                Ring {
                    value: 9,
                    width: 5.5,
                    color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
                    has_text: false,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 3.0,
                    hit_color: WebColor { hex: String::from("green"), alpha: 1.0 },
                },
                Ring {
                    value: 8,
                    width: 10.5,
                    color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 2.7,
                    hit_color: WebColor { hex: String::from("yellow"), alpha: 1.0 },
                },
                Ring {
                    value: 7,
                    width: 15.5,
                    color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 2.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 6,
                    width: 20.5,
                    color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 1.5,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 5,
                    width: 25.5,
                    color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 1.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 4,
                    width: 30.5,
                    color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("#ffffff"), alpha: 1.0 },
                    zoom: 1.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 3,
                    width: 35.5,
                    color: WebColor { hex: String::from("white"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("black"), alpha: 1.0 },
                    zoom: 1.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 2,
                    width: 40.5,
                    color: WebColor { hex: String::from("white"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("black"), alpha: 1.0 },
                    zoom: 1.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
                Ring {
                    value: 1,
                    width: 45.5,
                    color: WebColor { hex: String::from("white"), alpha: 1.0 },
                    has_text: true,
                    text_color: WebColor { hex: String::from("black"), alpha: 1.0 },
                    zoom: 1.0,
                    hit_color: WebColor { hex: String::from("#00bffF"), alpha: 1.0 },
                },
            ],
            rings_draw_only: vec![],
            default_hit_color: WebColor { hex: String::from("#000000"), alpha: 1.0 },
            default_zoom: 1.0,
            min_zoom: 0.3,
            inner_ten: 200,
            trial_corner_color: WebColor { hex: String::from("#0f0"), alpha: 0.7 },
            bullet_diameter: 4.5,
        },
        parts: vec![
            DisciplinePart {
                id: String::from("probe"),
                name: String::from("Probe"),
                has_trial_corner: true,
                main_part: false,
                enable_reset_to_new_target: true,
                series_length: 10,
                number_of_shots: 40,
                show_infos: true,
                count_mode: PartCountMode::Tenth,
                time: Time::None,
                average: PartAverage::Average { number_of_shots: 40 },
                exit_type: PartExitType::Always,
            },
        ],
    }
}


pub fn lg_target() -> Target {
    Target {
        title: String::from("LG 10m"),
        rings: vec![
            Ring {
                value: 10,
                width: 0.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 9,
                width: 5.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 8,
                width: 10.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 7,
                width: 15.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 6,
                width: 20.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 5,
                width: 25.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 4,
                width: 30.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 3,
                width: 35.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 2,
                width: 40.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
            Ring {
                value: 1,
                width: 45.5,
                color: WebColor { hex: String::from("000000"), alpha: 1.0 },
                has_text: false,
                text_color: WebColor { hex: String::from("ffffff"), alpha: 1.0 },
                zoom: 4.0,
                hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
            },
        ],
        rings_draw_only: vec![],
        default_hit_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
        default_zoom: 1.0,
        min_zoom: 0.3,
        inner_ten: 200,
        trial_corner_color: WebColor { hex: String::from("000000"), alpha: 1.0 },
        bullet_diameter: 4.5,
    }
}





// let target = helper::dsc_demo::lg_target();
// let discipline = helper::dsc_demo::lg_discipline();

// let shot = Shot::from_cartesian_coordinates(100, 100, &target);
// println!("{:?}", shot);

// let shot1 = Shot::from_cartesian_coordinates(-100, -100, &target);
// println!("{:?}", shot1);
//
// let shot2 = Shot::from_cartesian_coordinates(100, -100, &target);
// println!("{:?}", shot2);
//
// let shot3 = Shot::from_cartesian_coordinates(-100, 100, &target);
// println!("{:?}", shot3);
//
// let shot4 = Shot::from_cartesian_coordinates(0, 100, &target);
// println!("{:?}", shot4);
//
// let shot5 = Shot::from_cartesian_coordinates(100, 0, &target);
// println!("{:?}", shot5);
//
// let shot6 = Shot::from_cartesian_coordinates(0, 0, &target);
// println!("{:?}", shot6);
//
// let shot7 = Shot::from_cartesian_coordinates(1, 0, &target);
// println!("{:?}", shot7);
//
// let shot8 = Shot::from_cartesian_coordinates(2500, 0, &target);
// println!("{:?}", shot8);
//
// let shot9 = Shot::from_cartesian_coordinates(2501, 0, &target);
// println!("{:?}", shot9);
//
// let shot10 = Shot::from_cartesian_coordinates(5000, 5000, &target);
// println!("{:?}", shot10);
