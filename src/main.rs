use std::mem::swap;

use macroquad::prelude::*;


#[derive(Default,)]
struct Rayrectinfo {
    hit: bool,
    contact_point: Vec2,
    contact_normal: Vec2,
    t_hit_near: f32,
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
    let mut ray = Ray {
        start: vec2(800., 350.),
        direction: Vec2::ZERO,
    };

    let mut player = DynamicRectangle {
        rect: Rect::new(400., 300., 24., 50.),
        velocity: Vec2::ZERO,
    };

    let world = vec![
        Rect::new(-800., -50., 100., 100.),
        Rect::new(900., 300., 100., 100.),
        Rect::new(1000., 300., 100., 100.),
        Rect::new(850., 0., 100., 100.),
        Rect::new(1000., 100., 100., 100.),
        Rect::new(1000., 200., 100., 100.),
    ];

    loop {
        //logic
        wrap_position(&mut player.rect);
        let delta = get_frame_time();
        let mouse_pos:Vec2 = mouse_position().into();

        if is_mouse_button_down(MouseButton::Left) {
            player.velocity += (mouse_pos - ray.start).normalize_or_zero() * 10.0;
        }
        if is_mouse_button_down(MouseButton::Right) {
            ray.start = mouse_pos;
        }
        ray.direction = mouse_pos - ray.start;

        let mut collisions_with: Vec<(usize, f32)> = world
            .iter()
            .enumerate()
            .filter_map(|(i, _)| {
                let info = looping_dynamic_rect_vs_rect(&world[i], &player, delta, screen_width(), screen_height());
                if info.hit { Some((i, info.t_hit_near)) } else { None }
            })
            .collect();
        
        collisions_with.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (i, _) in collisions_with {
            let rayrectinfo = looping_dynamic_rect_vs_rect(&world[i], &player, delta, screen_width(), screen_height());
            if rayrectinfo.hit {
                player.velocity += rayrectinfo.contact_normal * player.velocity.abs() * (1.0 - rayrectinfo.t_hit_near);
            }
        }

        //draw
        player.rect.x += player.velocity.x * delta;
        player.rect.y += player.velocity.y * delta;

        clear_background(Color::new(0.0, 0.0, 0.1, 1.0));
        draw_text(&format!("{}", get_fps()), 0.0, 40.0, 40.0, WHITE);

        for element in &world {
            let screen_width = screen_width();
            let screen_height = screen_height();
        
            let x_offsets = [0.0, screen_width, -screen_width];
            let y_offsets = [0.0, screen_height, -screen_height];
        
            for &x_offset in &x_offsets {
                for &y_offset in &y_offsets {
                    let x = element.x + x_offset;
                    let y = element.y + y_offset;
        
                    // Check if the rectangle is at least partially visible
                    if x < screen_width && x + element.w > 0.0 &&
                       y < screen_height && y + element.h > 0.0 {
                        draw_rectangle_lines(x, y, element.w, element.h, 10.0, BEIGE);
                    }
                }
            }
        }
        draw_rectangle_lines(player.rect.x, player.rect.y, player.rect.w, player.rect.h, 10.0, PINK);
        draw_line(ray.start.x, ray.start.y, mouse_pos.x, mouse_pos.y, 4., GOLD);

        next_frame().await;
    }
}

fn looping_dynamic_rect_vs_rect(rect: &Rect, dynrect: &DynamicRectangle, delta: f32, width: f32, height: f32) -> Rayrectinfo {
    let x_offsets = [0.0, width, -width];
    let y_offsets = [0.0, height, -height];
    
    let mut earliest_hit = Rayrectinfo::default();

    for &x_offset in &x_offsets {
        for &y_offset in &y_offsets {
            let shifted_rect = Rect {
                x: rect.x + x_offset,
                y: rect.y + y_offset,
                w: rect.w,
                h: rect.h,
            };

            let info = dynamic_rect_vs_rect(&shifted_rect, dynrect, delta);
            if info.hit && (earliest_hit.hit == false || info.t_hit_near < earliest_hit.t_hit_near) {
                earliest_hit = info;
                break;
            }
        }
    }

    earliest_hit
}

fn dynamic_rect_vs_rect(rect: &Rect, dynrect: &DynamicRectangle, delta: f32) -> Rayrectinfo {
    if dynrect.velocity == Vec2::ZERO {
        return Rayrectinfo::default();
    }


    let expanded_rect = Rect::new(
        rect.x - dynrect.rect.w / 2.0,
        rect.y - dynrect.rect.h / 2.0,
        rect.w + dynrect.rect.w,
        rect.h + dynrect.rect.h,
    );

    let ray = Ray {
        start: dynrect.rect.point() + dynrect.rect.size() / 2.0,
        direction: dynrect.velocity * delta,
    };
    
    let mut rayrectinfo = ray_vs_rect(&ray, &expanded_rect);
    if rayrectinfo.hit && (rayrectinfo.t_hit_near >= 0.0 && rayrectinfo.t_hit_near <= 1.0) {
        return rayrectinfo; 
    }

    rayrectinfo.hit = false;
    rayrectinfo
}





fn ray_vs_rect(ray: &Ray, rect: &Rect) -> Rayrectinfo {
    let mut t_near = (rect.point() - ray.start) / ray.direction;
    let mut t_far = (rect.point() + rect.size() - ray.start) / ray.direction;

    if t_near.x > t_far.x { swap(&mut t_near.x, &mut t_far.x); }
    if t_near.y > t_far.y { swap(&mut t_near.y, &mut t_far.y); }

    if t_far.x.is_nan() || t_far.y.is_nan() || t_near.x.is_nan() || t_near.y.is_nan() { return Rayrectinfo::default(); }
    if t_near.x > t_far.y || t_near.y > t_far.x { return Rayrectinfo::default(); }

    let t_hit_near = t_near.x.max(t_near.y);
    if t_hit_near < 0.0 || t_far.x.min(t_far.y) < 0.0 { return Rayrectinfo::default(); }

    let contact_normal = if t_near.x > t_near.y {
        if ray.direction.x.is_sign_negative() { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) }
    } else {
        if ray.direction.y.is_sign_negative() { vec2(0.0, 1.0) } else { vec2(0.0, -1.0) }
    };

    Rayrectinfo {
        hit: true,
        contact_point: ray.start + t_hit_near * ray.direction,
        contact_normal,
        t_hit_near,
    }
}



fn wrap_position(rect: &mut Rect) {
    let (w, h) = (screen_width(), screen_height());
    rect.x = (rect.x + w) % w;
    rect.y = (rect.y + h) % h;
}