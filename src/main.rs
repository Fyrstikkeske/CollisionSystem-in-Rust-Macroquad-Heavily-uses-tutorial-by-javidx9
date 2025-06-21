use macroquad::prelude::*;

#[derive(Clone)]
struct ConvexShape {
    vertices: Vec<Vec2>,
}

struct Simplex{
    a: Vec2,
    b: Vec2,
    c: Vec2,

    count:i32,
}

#[macroquad::main("GJK Collision Detection")]
async fn main() {
    let camera = Camera2D {
        target: vec2(0.0, 0.0), // Where the camera is looking
        zoom: vec2(3.0 / screen_width(), 3.0 / screen_height()), // Fit screen
        ..Default::default()
    };

    let square = vec![
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, -20.0),
        Vec2::new(20.0, 20.0),
        Vec2::new(-20.0, 20.0),
    ];

    let staticsquare = vec![
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, -20.0),
        Vec2::new(20.0, 20.0),
        Vec2::new(-20.0, 20.0),
    ];



    let mut staticshape = ConvexShape{
        vertices: staticsquare,
    };
    staticshape.vertices = move_verticee_list(&staticshape.vertices, Vec2{x: 100., y:100.});
    let mut rotation:f32 = 0.0;

    loop {
        set_camera(&camera);
        clear_background(LIGHTGRAY);
        let mut shape = ConvexShape{
            vertices: rotate(&square, rotation),
        };
        // Handle input
        if is_key_down(KeyCode::R) {
            rotation += 0.1;
        }

        // Update shape_b to follow mouse
        let mousepos: Vec2 = camera.screen_to_world(mouse_position().into());
        shape.vertices = move_verticee_list(&shape.vertices, mousepos);

        // Draw shapes
        draw_circle(0.0,0.0, 2.0, BLUE);
        draw_shape(&shape, RED);
        draw_shape(&staticshape, RED);
        let minkowski_sum = make_minkowsky_sum(&shape,&staticshape);
        draw_dots(&minkowski_sum, RED);



        let mut direction = get_average(&shape.vertices) - get_average(&staticshape.vertices);
        let mut dot_produkt:Vec2 = gjk_get_support_function(&shape.vertices, direction) - gjk_get_support_function(&staticshape.vertices, -direction);
        direction = -dot_produkt;
        

        draw_circle(dot_produkt.x,dot_produkt.y, 3.0, BLUE);
        draw_circle(get_average(&shape.vertices).x, get_average(&shape.vertices).y, 3.0, BLUE);
        draw_circle(get_average(&staticshape.vertices).x,get_average(&staticshape.vertices).y, 3.0, BLUE);

        let mut continuelopp = true;
        while continuelopp{
            dot_produkt = gjk_get_support_function(&shape.vertices, direction) - gjk_get_support_function(&staticshape.vertices, -direction);
            draw_circle(dot_produkt.x,dot_produkt.y, 3.0, YELLOW);
            continuelopp = false;
        }

        // Draw status
        draw_text(
            &format!("Minkowski test",),
            20.0,
            20.0,
            30.0,
            BLACK,
        );
        set_default_camera();
        next_frame().await;
    }
}

fn get_average(list: &Vec<Vec2>) -> Vec2{
    let mut average = Vec2{x: 0.0, y:0.0};

    for i in list {
        average += *i;
    }
    average = average / list.len() as f32;
    return average;
}

fn move_verticee_list(list: &Vec<Vec2>, translate: Vec2) -> Vec<Vec2> {
    let mut newlist: Vec<Vec2> = vec![];

    for i in 0..list.len() {
        newlist.push(list[i] + translate);
    }
    return newlist;
}

fn gjk_get_support_function(list: &Vec<Vec2>, direction: Vec2) -> Vec2{
    let mut largest_vertex: Vec2 = list[0];
    let mut largest_dot: f32 = list[0].dot(direction);

    for i in list{
        let dot_product = i.dot(direction);
        if dot_product > largest_dot{
            largest_dot = dot_product;
            largest_vertex = *i;
        }
    }
    return largest_vertex
}



fn make_minkowsky_sum(shape1: &ConvexShape, shape2: &ConvexShape) -> Vec<Vec2>{
    let mut newlist: Vec<Vec2> = Vec::with_capacity(shape1.vertices.len() * shape2.vertices.len());

    for list1 in 0..shape1.vertices.len() {
        for list2 in 0..shape2.vertices.len() {
            newlist.push(shape1.vertices[list1] - shape2.vertices[list2]);
        }
    }
    return newlist;
}

fn rotate(list: &Vec<Vec2>, angle: f32) -> Vec<Vec2> {
    let mut newlist: Vec<Vec2> = Vec::with_capacity(list.len());
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    for vec in list {
        let x_new = vec.x * cos_theta - vec.y * sin_theta;
        let y_new = vec.x * sin_theta + vec.y * cos_theta;
        newlist.push(Vec2 { x: x_new, y: y_new });
    }

    newlist
}

fn draw_shape(shape: &ConvexShape, color: Color) {
    let vertices: Vec<Vec2> = shape
        .vertices
        .iter()
        .map(|v| *v)
        .collect();

    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % vertices.len()];
        draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, color);
    }
}


fn draw_dots(list: &Vec<Vec2>, color: Color) {
    for i in list {
        draw_circle(i.x, i.y, 2.0, color);
    }
}
