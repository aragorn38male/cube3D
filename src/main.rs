//use kiss3d::camera::FirstPerson;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::text::Font;
use kiss3d::window::Window;
use nalgebra::{Point2, Point3, Translation3, UnitQuaternion, Vector3};
use rand::Rng;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::Path;

struct CubeData {
    position: Translation3<f32>,
    rotation: UnitQuaternion<f32>,
}

fn backup_cube(cube: &SceneNode, storage: &mut HashMap<String, CubeData>, id: &str) {
    storage.insert(
        id.to_string(),
        CubeData {
            position: cube.data().local_translation(),
            rotation: cube.data().local_rotation(),
        },
    );
}

fn restore_cube(cube: &mut SceneNode, storage: &HashMap<String, CubeData>, id: &str) {
    if let Some(data) = storage.get(id) {
        cube.set_local_translation(data.position);
        cube.set_local_rotation(data.rotation);
        cube.set_visible(true);
    }
}

fn main() {
    let mut cube_storage: HashMap<String, CubeData> = HashMap::new();

    let mut rng = rand::rng();

    let mut window = Window::new("cube3D");
    window.set_light(Light::StickToCamera);
    window.hide_cursor(true);

    /*
        let eye = Point3::new(0.0, 0.0, 4.0f32);
        let at = Point3::origin();
        let mut first_person = FirstPerson::new(eye, at);
    */

    let mut ang = 0;
    let mut randrot = 0;

    let mut direction = 0;
    let mut choice = 0;

    let path_obj_cube = Path::new("media/cube/cube.obj");
    let path_mtl_cube = Path::new("media/cube/cube.mtl");

    let mut cubes = Vec::new();

    let col = (
        rng.random_range(0.0..=1.0),
        rng.random_range(0.0..=1.0),
        rng.random_range(0.0..=1.0),
    );

    for _z in 0..3 {
        for _y in 0..3 {
            for _x in 0..3 {
                // BODY / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                let mut c =
                    window.add_obj(&path_obj_cube, &path_mtl_cube, Vector3::new(0.1, 0.1, 0.1));
                c.set_local_translation(Translation3::new(
                    0.50 * _x as f32 - 0.750,
                    0.50 * _y as f32 - 0.750,
                    0.50 * _z as f32 - 0.750,
                ));
                c.set_color(col.0, col.1, col.2);

                /*
                                //  TOP / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _y == 2 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.append_rotation(&UnitQuaternion::from_axis_angle(
                                        &Vector3::x_axis(),
                                        -PI as f32 / 2.0,
                                    ));
                                    p.set_local_translation(Translation3::new(0.25, 0.501, 0.25));
                                    p.set_color(1.0, 0.0, 0.0);
                                }

                                // BOTTOM / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _y == 0 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.append_rotation(&UnitQuaternion::from_axis_angle(
                                        &Vector3::x_axis(),
                                        -PI as f32 / 2.0,
                                    ));
                                    p.set_local_translation(Translation3::new(0.25, -0.001, 0.25));
                                    p.set_color(1.0, 1.0, 0.0);
                                }

                                //  FRONT / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _z == 0 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.set_local_translation(Translation3::new(0.25, 0.25, -0.001));
                                    p.set_color(0.0, 1.0, 0.0);
                                }

                                //  BACK / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _z == 2 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.set_local_translation(Translation3::new(0.25, 0.25, 0.501));

                                    p.set_color(0.0, 1.0, 1.0);
                                }

                                //  LEFT / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _x == 2 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.append_rotation(&UnitQuaternion::from_axis_angle(
                                        &Vector3::y_axis(),
                                        -PI as f32 / 2.0,
                                    ));

                                    p.set_local_translation(Translation3::new(0.501, 0.25, 0.25));
                                    p.set_color(0.0, 0.0, 1.0);
                                }

                                //  RIGHT / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
                                if _x == 0 {
                                    let mut p = c.add_quad(4.0, 4.0, 1, 1);
                                    p.append_rotation(&UnitQuaternion::from_axis_angle(
                                        &Vector3::y_axis(),
                                        -PI as f32 / 2.0,
                                    ));

                                    p.set_local_translation(Translation3::new(-0.001, 0.25, 0.25));
                                    p.set_color(1.0, 0.0, 1.0);
                                }

                */

                cubes.push(c);
            }
        }
    }

    let mut n = 0;
    for cube in &mut cubes {
        let c = format!("cube{}", n);
        backup_cube(&cube, &mut cube_storage, &c);
        n += 1;
    }

    //     while !window.should_close() {
    //      window.render();
    //        window.render_with_camera(&mut first_person);

    while window.render() {
        window.draw_text(
            "to be continued...",
            &Point2::new(50.0, 50.0),
            90.0,
            &Font::default(),
            &Point3::new(col.2, col.1, col.0),
        );

        /*
                let a = Point3::new(-9.0, 0.0, 0.0);
                let b = Point3::new(9.0, 0.0, 0.0);
                let c = Point3::new(0.0, -9.0, 0.0);
                let d = Point3::new(0.0, 9.0, 0.0);
                let e = Point3::new(0.0, 0.0, -9.0);
                let f = Point3::new(0.0, 0.0, 9.0);

                window.set_line_width(2.0);
                window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
                window.draw_line(&c, &d, &Point3::new(0.0, 1.0, 0.0));
                window.draw_line(&e, &f, &Point3::new(0.0, 0.0, 1.0));
        */
        if ang == 90 {
            ang = 0;
            std::thread::sleep(std::time::Duration::from_millis(100));
            randrot = rng.random_range(0..9);

            let mut i = 0;
            for cube in &mut cubes {
                let c = format!("cube{}", i);
                restore_cube(cube, &cube_storage, &c);
                i += 1;
            }
        } else {
            let axis = match randrot {
                0 => Vector3::x_axis(),
                1 => Vector3::x_axis(),
                2 => Vector3::x_axis(),
                3 => Vector3::y_axis(),
                4 => Vector3::y_axis(),
                5 => Vector3::y_axis(),
                6 => Vector3::z_axis(),
                7 => Vector3::z_axis(),
                _ => Vector3::z_axis(),
            };

            let rotation = if choice == 0 {
                UnitQuaternion::from_axis_angle(&axis, -PI / 180.0)
            } else {
                UnitQuaternion::from_axis_angle(&axis, PI / 180.0)
            };
            direction += 1;

            if direction % 90 == 0 {
                choice = rand::rng().random_range(0..=1);
            }

            ang += 1;
            for cube in &mut cubes {
                let translation = cube.data().local_translation();

                match randrot {
                    0 => {
                        if translation.x == -0.75 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    1 => {
                        if translation.x == -0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    2 => {
                        if translation.x == 0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }

                    3 => {
                        if translation.y == -0.75 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    4 => {
                        if translation.y == -0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    5 => {
                        if translation.y == 0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    6 => {
                        if translation.z == -0.75 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    7 => {
                        if translation.z == -0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }
                    8 => {
                        if translation.z == 0.25 {
                            cube.append_rotation(&rotation);
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}
