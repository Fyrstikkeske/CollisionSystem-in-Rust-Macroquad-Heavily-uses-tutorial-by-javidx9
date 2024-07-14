use std::mem::swap;

use macroquad::prelude::*;

struct Rayrectinfo{
    hit: bool,
    contact_point:Vec2,
    contact_normal:Vec2,
    t_hit_near:f32,
}

struct DynamicRectangle{
    rect:Rect,
    velocity:Vec2,
}




struct Ray{
    start:Vec2,
    direction:Vec2
}


#[macroquad::main("collision")]
async fn main() {
    let mut ray= Ray { start: Vec2{x: 800., y: 350.0}, direction: Vec2{x: 0., y: 0.0}};

    let mut player = DynamicRectangle{
        rect: Rect{x: 400.,y: 300., w: 24.,h: 50.,},
        velocity: Vec2::ZERO
    };

    let world: Vec<Rect> = vec![
        Rect{x: 800.,y: 300., w: 100.,h: 100.,},
        Rect{x: 900.,y: 300., w: 100.,h: 100.,},
        Rect{x: 1000.,y: 300., w: 100.,h: 100.,},
        Rect{x: 850.,y: 0., w: 100.,h: 100.,},
        Rect{x: 1000.,y:100., w: 100.,h: 100.,},
        Rect{x: 1000.,y:200., w: 100.,h: 100.,},
    ];

    loop {
        clear_background(Color{a: 1.0,r: 0.0, g: 0.0, b: 0.1});
        draw_text(format!("{}", get_fps()).as_str(), 0.0, 40.0, 40.0, WHITE);
        let delta = get_frame_time();
        let mouse_pos:Vec2 = mouse_position().into();

        if is_mouse_button_down(MouseButton::Left) {
            player.velocity += (mouse_pos - ray.start).normalize_or_zero() * 10.0;
        }

        if is_mouse_button_down(MouseButton::Right) {
            ray.start = mouse_pos
        }

        ray.direction = mouse_pos - ray.start;

        let mut collisions_with:Vec<(usize,f32)> = vec![];
        
        for (i, element) in world.iter().enumerate(){
            let rayrectinfo = dynamic_rect_vs_rect(element, &player, &delta);
            
            if rayrectinfo.hit{
                collisions_with.push((i,rayrectinfo.t_hit_near));
            }
        }

        collisions_with.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for round in collisions_with{
            let element = world[round.0];
            let rayrectinfo = dynamic_rect_vs_rect(&element, &player, &delta);
            if rayrectinfo.hit{
                player.velocity += rayrectinfo.contact_normal * player.velocity.abs() * (1.0-rayrectinfo.t_hit_near);
            }
        }

        player.rect.x += player.velocity.x*delta;
        player.rect.y += player.velocity.y*delta;

        for element in &world{
            rectangle(*element, BEIGE);
        }

        rectangle(player.rect, PINK);
        draw_line(ray.start.x, ray.start.y, mouse_pos.x, mouse_pos.y, 4., GOLD);

        
        next_frame().await
    }
}

fn dynamic_rect_vs_rect(
    rect:&Rect,
    dynrect: &DynamicRectangle,
    delta: &f32,
        ) -> Rayrectinfo
{
    let mut rayrectinfo = Rayrectinfo{hit: false, contact_point: Vec2{x:0.0,y: 0.0}, contact_normal: Vec2{x:0.0,y: 0.0}, t_hit_near: 0.0};

    if dynrect.velocity.x == 0.0 && dynrect.velocity.y == 0.0{
        return rayrectinfo;
    }

    
    let exp_rect_pos = rect.point() - dynrect.rect.size() / 2.;
    let exp_rect_size = rect.size() + dynrect.rect.size();
    let expanded_target:Rect = Rect { x: exp_rect_pos.x, y: exp_rect_pos.y, w: exp_rect_size.x, h: exp_rect_size.y };

    rayrectinfo = ray_vs_rect(
        &Ray{ start: dynrect.rect.point() + dynrect.rect.size()/2.0, direction: dynrect.velocity * *delta},
        &expanded_target
    );

    if rayrectinfo.hit{
        if rayrectinfo.t_hit_near <= 1.0 && rayrectinfo.t_hit_near >= 0.0{ 
            rayrectinfo.hit = true;
            return rayrectinfo;
    }}

    rayrectinfo.hit = false;
    rayrectinfo
}





fn ray_vs_rect(
    ray:&Ray,
    rect: &Rect,
        ) -> Rayrectinfo{
    let mut rayrectinfo = Rayrectinfo{
        hit: false,
        contact_point: Vec2{x:0.0,y: 0.0}, 
        contact_normal: Vec2{x:0.0,y: 0.0}, 
        t_hit_near: 0.0};

    let mut t_near = (rect.point() - ray.start) / ray.direction;
    let mut t_far = (rect.point() + rect.size() - ray.start) / ray.direction;
    
    if t_near.x > t_far.x { swap( &mut t_near.x, &mut t_far.x)};
    if t_near.y > t_far.y { swap( &mut t_near.y, &mut t_far.y)};
    
    if t_far.y.is_nan() || t_far.x.is_nan() {return rayrectinfo};
    if t_near.y.is_nan() || t_near.x.is_nan() {return rayrectinfo};

    if t_near.x > t_far.y || t_near.y > t_far.x {return rayrectinfo};

    rayrectinfo.t_hit_near = f32::max(t_near.x, t_near.y);
    let t_hit_far = f32::min(t_far.x, t_far.y);

    if t_hit_far <0.0 {return rayrectinfo};

    rayrectinfo.contact_point = ray.start + rayrectinfo.t_hit_near * ray.direction;

    if t_near.x > t_near.y{
        if ray.direction.x < 0.0{
            rayrectinfo.contact_normal = Vec2 {x: 1.0,y: 0.0}
        }else{
            rayrectinfo.contact_normal = Vec2 {x: -1.0,y: 0.0}
        }
    }else if t_near.x < t_near.y {
        if ray.direction.y < 0.0{
            rayrectinfo.contact_normal = Vec2 {x: 0.0,y: 1.0}
        }else{
            rayrectinfo.contact_normal = Vec2 {x: 0.0,y: -1.0}
        }
    }
    rayrectinfo.hit = true;
    rayrectinfo
}




fn rectangle(rect: Rect, color: Color){
    draw_rectangle_lines( rect.x, rect.y, rect.w, rect.h, 10.0,color)
}