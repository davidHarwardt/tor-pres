#![windows_subsystem = "windows"]

mod utils;

use std::{time::Duration, ops::{Add, Mul}};

use nannou::prelude::*;
use rand::prelude::*;
use utils::{ColorExt, lerp};

trait Scene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect);
    fn update(&mut self, app: &App, dt: Duration, t: Duration);

    fn reset(&mut self) {  }
    fn next_step(&mut self) -> NextStep { NextStep::Finished }
}

enum NextStep {
    Running,
    Finished,
}
impl NextStep {
    fn is_finished(&self) -> bool { matches!(self, NextStep::Finished) }
}

struct TitleScene {
    points: Vec<(Vec2, Vec<usize>)>,
    walkers: Vec<(usize, usize, f32, f32)>,
}

impl TitleScene {
    fn new() -> Self {
        const NUM_NODES: usize = 10;
        const MAX_DIST: f32 = 0.115;
        let dim = vec2(1.0, 1.0);
        let mut rng = StdRng::seed_from_u64(42);

        let mut points = Vec::new();
        for i in 0..NUM_NODES {
            for j in 0..NUM_NODES {
                let p = vec2(i as _, j as _) / (NUM_NODES as f32) * dim;
                let room = dim / (NUM_NODES as f32);
                let center = p + room / 2.0;

                let off = (rng.gen::<Vec2>() - 0.5) * room;
                points.push((center + off - 0.5 * dim, Vec::new()));
            }
        }

        for i in 0..points.len() {
            for j in 0..points.len() {
                if points[i].0.distance(points[j].0) < MAX_DIST {
                    points[i].1.push(j);
                }
            }
        }
        const WALKER_COUNT: usize = 100;
        let mut walkers = Vec::new();
        for _ in 0..WALKER_COUNT {
            let start = rng.gen_range(0..WALKER_COUNT);
            let end = loop {
                let v = rng.gen_range(0..WALKER_COUNT);
                if v != start { break v }
            };
            walkers.push((start, end, 0.0, rng.gen_range(0.6..=1.2)));
        }

        Self { points, walkers }
    }
}

impl Scene for TitleScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        let t = app.time;
        for (point, conns) in self.points.iter() {
            let pos = *point * frame.w();
            if frame.contains(pos) {
                layered_point(draw, 6, 3, *point * frame.w(), 20.0, rgba(0.5, 0.5, 0.5, 0.4), t, 0.008, 2.0);
            }

            for conn in conns.iter() {
                draw.line().start(*point * frame.w()).end(self.points[*conn].0 * frame.w()).color(gray(0.3).into_format().with_alpha(0.3)).weight(3.0);
            }
        }

        for (from, to, wt, _) in self.walkers.iter() {
            let start = self.points[*from].0 * frame.w();
            let end = self.points[*to].0 * frame.w();
            draw.ellipse().xy(lerp(start, end, *wt)).radius(6.0).color(DARKRED.with_alpha(0.9));
        }

        draw.scale(0.98).rect().w_h(1000.0, 150.0).color(DARKRED.with_alpha(0.95)).y(-10.0);
        draw.scale(0.98).text("The TOR network").font_size(100).y_align_text(text::Align::Middle).width(frame.w()).color(gray(0.4));
        draw.scale(0.99).text("The TOR network").font_size(100).y_align_text(text::Align::Middle).width(frame.w()).color(gray(0.6));

        for (point, _) in self.points.iter() {
            let pos = *point * frame.w();
            if frame.contains(pos) {
                layered_point(draw, 3, 0, *point * frame.w(), 20.0, rgba(0.5, 0.5, 0.5, 0.4), t, 0.008, 2.0);
            }
        }

        draw.rect().w_h(1000.0, 150.0).color(DARKRED.with_alpha(0.5)).y(-10.0);
        draw.text("The TOR network").font_size(100).y_align_text(text::Align::Middle).width(frame.w()).color(gray(0.8));
    }

    fn update(&mut self, app: &App, dt: Duration, _t: Duration) {
        let mut rng = rand::thread_rng();
        for (from, to, wt, speed) in self.walkers.iter_mut() {
            *wt += dt.as_secs_f32() * *speed;
            if *wt > 1.0 {
                *from = *to;
                *to = self.points[*from].1[rng.gen_range(0..self.points[*from].1.len())];
                *wt = 0.0;
            }
        }
    }
}


fn layered_point(draw: &Draw, layers: usize, off: usize, pos: Vec2, r: f32, color: impl Into<Rgba>, t: f32, shrink: f32, r_shrink: f32) {
    let mut color: Rgba = color.into();
    let og_alpha = color.alpha;
    for layer in (off..layers).rev() {
        let layer = layer as f32;
        let v_t = t / 10.0;
        // let off = vec2(v_t.sin(), v_t.cos()) * 500.0;
        let off = vec2(0.0, 0.0);
        let p = ((pos + off) * (1.0 - (layer * shrink))) - off;

        color.alpha = (1.0 - layer / layers as f32) * og_alpha;
        draw.ellipse().radius(r - (layer * r_shrink)).xy(p).color(color);
    }
}

fn draw_slide(text: &str, draw: &Draw, frame: &Rect) {
    draw.text(text).width(500.0).xy(frame.bottom_left() + vec2(280.0, 40.0)).color(gray(0.4).into_format().with_alpha(0.3)).font_size(30).left_justify();
}


struct GeneralScene {

}
impl GeneralScene {
    fn new() -> Self {
        Self {

        }
    }
}
impl Scene for GeneralScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide("TOR - general", draw, frame);
        draw.text(r#"
        - short for "The Onion Router"
        - network ontop of internet
        - free to use
        - operated by volunteers
        - mostly decentralized
        - used to conceal location and usage
        "#).w_h(1000.0, 1000.0).left_justify().font_size(50).color(gray(0.8));
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {

    }
}

fn draw_title_block(vtext: &str, rect: Rect, fontsize: u32, draw: &Draw) -> Rect {
    let text = text(vtext).center_justify().font_size(fontsize);
    let t = text.build(rect);
    let br = t.bounding_rect().pad(-50.0).pad_left(-50.0).pad_right(-50.0);
    draw.scale(1.02).rect().xy(br.xy()).wh(br.pad(-1.0).pad_right(-20.0).pad_left(-20.0).wh()).color(DARKRED.with_alpha(0.2));
    draw.rect().xy(br.xy()).wh(br.wh()).color(DARKRED.with_alpha(0.6));

    for scale in 1..3 {
        let scale = 1.0 - ((scale as f32) * 0.005);
        draw.scale(scale).text(vtext).xy(rect.xy()).wh(rect.wh()).center_justify().color(gray(0.8 - (scale as f32 * 0.2))).font_size(fontsize);
    }
    draw.path().fill().events(t.path_events()).color(gray(0.8));
    br
}

struct QuoteScene {
    quote_text: String,
    source: String,
    name: String,
    t: f32,
}
impl QuoteScene {
    fn new(quote_text: impl Into<String>, source: impl Into<String>, name: impl Into<String>) -> Self {
        let quote_text = quote_text.into();
        let source = source.into();
        let name = name.into();
        Self { quote_text, source, name, t: 0.0 }
    }
}
impl Scene for QuoteScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        let text_rect = Rect::from_w_h(1000.0, 400.0f32);
        {
            // let text = text(&self.quote_text).center_justify().font_size(60);
            // let t = text.build(text_rect);
            // let br = t.bounding_rect().pad(-50.0).pad_left(-50.0).pad_right(-50.0);
            // draw.rect().xy(br.xy()).wh(br.wh()).color(DARKRED.with_alpha(0.8));
            // draw.path().fill().events(t.path_events()).color(gray(0.8));

            let br = draw_title_block(&self.quote_text, text_rect, 60, &draw.scale(self.t * 4.0 * ((self.t * 4.0 * 2.0 * PI).sin().abs() * 0.2 + 0.8)));
            let source_rect = Rect::from_w_h(500.0, 40.0).bottom_right_of(br.pad(-50.0));
            draw.text(&self.source).align_text_bottom().right_justify().xy(source_rect.xy()).wh(source_rect.wh()).color(gray(0.6)).font_size(40);
        }
        draw_slide(&self.name, draw, frame);
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {
        self.t += dt.as_secs_f32();
        self.t = self.t.min(0.25);
    }
    
    fn reset(&mut self) {
        self.t = 0.0;
    }
}

struct TimelineScene {
    name: String,
    events: Vec<TimelineEvent>,
    current_event: f32,
    target: i32,
}
struct TimelineEvent {
    year: String,
    label: String,
    image: Option<wgpu::Texture>,
}
impl TimelineEvent {
    fn new(year: impl Into<String>, label: impl Into<String>, image: Option<wgpu::Texture>) -> Self {
        let label = label.into();
        let year = year.into();
        Self { year, label, image }
    }
}
impl TimelineScene {
    fn new(events: Vec<TimelineEvent>, name: impl Into<String>) -> Self {
        let current_event = -1.0;
        let name = name.into();
        let target = -1;
        Self { name, events, current_event, target }
    }

}
impl Scene for TimelineScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide(&self.name, draw, frame);
        let r = frame.pad(50.0);
        draw.line().start(r.mid_left()).end(r.mid_right()).weight(10.0).color(gray(0.7));

        for (i, ev) in self.events.iter().enumerate() {
            let idx = i as f32;
            let ev_width = frame.w() * 0.75;
            let ev_pos = r.xy() + vec2((idx - self.current_event) * ev_width, 0.0);
            let center_dist = (idx - self.current_event).abs().min(1.0);
            let size = (1.0 - center_dist * 2.0).max(0.0);
            if size > 0.01 {
                let marker_start = vec2(0.0, 50.0) * size;
                draw.line().start(ev_pos + marker_start).end(ev_pos - marker_start).weight(15.0 * size).color(gray(0.8));

                let rt = draw_title_block(&ev.label, Rect::from_xy_wh((ev_pos + vec2(0.0, 200.0) - vec2(0.0, (1.0 - size) * 200.0)) / size.max(0.01), vec2(500.0, 100.0)), 40, &draw.scale(size));
                let year_r = Rect::from_w_h(500.0, 200.0).mid_top_of(rt).shift_y(220.0);
                draw.scale(size).text(&ev.year).align_text_bottom().xy((year_r.xy() - vec2(0.0, (1.0 - size) * 300.0)) / size.max(0.01)).wh(year_r.wh()).color(gray(0.6)).font_size(30);
            }
        }
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {
        self.current_event = lerp(self.current_event, self.target as _, 0.75 * dt.as_secs_f32());
    }

    fn reset(&mut self) {
        self.current_event = -1.0;
        self.target = -1;
    }

    fn next_step(&mut self) -> NextStep {
        if self.target < self.events.len() as _ {
            self.target += 1;
            NextStep::Running
        } else { NextStep::Finished }
    }
}

struct FundingScene {
    sponsors: Vec<(String, Option<wgpu::Texture>)>,
}
impl FundingScene {
    fn new(app: &App) -> Self {
        let sponsors = vec![
            (String::from("Electronic Frontier Foundation"), img(app, include_bytes!("./assets/eff_logo.png"))),
            (String::from("US Bureau of Democracy, Human Rights and Labor"), img(app, include_bytes!("./assets/us_democracy_logo.png"))),
            (String::from("International Broadcasting Bureau"), img(app, include_bytes!("./assets/ibb-logo.gif"))),
            (String::from("Internews"), img(app, include_bytes!("./assets/internews-logo.jpg"))),
            (String::from("Human Rights Watch"), img(app, include_bytes!("./assets/human-rights-watch-logo.png"))),
            (String::from("University of Cambridge"), img(app, include_bytes!("./assets/cambridge-logo.png"))),
            (String::from("Google"), img(app, include_bytes!("./assets/google-logo.png"))),
            (String::from("NLnet"), img(app, include_bytes!("./assets/nlnet-logo.png"))),
        ];
        Self { sponsors }
    }
}
fn img(app: &App, bytes: &[u8]) -> Option<wgpu::Texture> {
    #[cfg(not(debug_assertions))] {
        let img = nannou::image::load_from_memory(bytes).expect("could not load image");
        Some(wgpu::Texture::from_image(app, &img))
    } #[cfg(debug_assertions)] {
        None
    }
}
impl Scene for FundingScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide("TOR - early funding", draw, frame);

        let dim = vec2(350.0, 350.0);
        let container = Rect::from_wh(dim * 4.0);
        for (i, (name, image)) in self.sponsors.iter().enumerate() {
            let pos = (vec2((i % 4) as _, (i / 4) as _) - vec2(2.0, 1.0)) * dim + dim / 2.0;
            let container = Rect::from_xy_wh(pos, dim).pad(50.0);
            draw.rect().xy(container.xy()).wh(container.wh()).color(gray(0.6).into_format().with_alpha(0.5));
            image.as_ref().map(|image| {
                draw.texture(image).xy(container.xy()).w(container.w()).h(container.h() * ((image.size()[1] as f32) / (image.size()[0] as f32)));
            });
            let text_rect = Rect::from_w_h(300.0, 50.0).mid_bottom_of(container).shift_y(-70.0);
            draw.text(&name).font_size(20).xy(text_rect.xy()).wh(text_rect.wh()).align_text_top();
        }
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {

    }
}

struct TargetValue<T> {
    value: T,
    target: T,
    speed: f32,
}
impl<T: Copy + Add<T, Output = T> + Mul<f32, Output = T>> TargetValue<T> {
    fn new(value: T, speed: f32) -> Self {
        let target = value;
        Self { value, target, speed }
    }
    fn goto(&mut self, target: T) { self.target = target }
    fn update(&mut self, dt: f32) { self.value = lerp(self.value, self.target, self.speed * dt) }
}
impl<T> std::ops::Deref for TargetValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl<T> std::ops::DerefMut for TargetValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}
struct Person {
    pos: TargetValue<Vec2>,
    scale: TargetValue<f32>,
    name: String,
}
impl Person {
    fn new(name: impl Into<String>, pos: Vec2) -> Self {
        let pos = TargetValue::new(pos, 5.2);
        let name = name.into();
        let scale = TargetValue::new(0.0, 5.2);
        Self { pos, name, scale }
    }
    fn draw(&self, draw: &Draw) -> Rect {
        draw_title_block(&self.name, Rect::from_xy_wh(*self.pos, vec2(500.0, 200.0)), 40, &draw.scale(*self.scale))
    }
    fn update(&mut self, dt: f32) {
        self.scale.update(dt);
        self.pos.update(dt);
    }
}
struct MotivationScene {
    alice: Person,
    bob: Person,
    carol: Person,
    step: u32,
    connections: (TargetValue<f32>, TargetValue<f32>),
    https: TargetValue<f32>,
    connection_id: TargetValue<f32>,
    alice_dead: TargetValue<f32>,
    tor_visible: TargetValue<f32>,
}

impl MotivationScene {
    fn new() -> Self {
        let alice = Person::new("Alice", vec2(0.0, 0.0));
        let bob = Person::new("Bob", vec2(0.0, 0.0));
        let carol = Person::new("Carol", vec2(0.0, 0.0));
        let step = 0;
        let connections = (TargetValue::new(0.0, 5.2), TargetValue::new(0.0, 5.2));
        let https = TargetValue::new(0.0, 5.2);
        let connection_id = TargetValue::new(0.0, 5.2);
        let alice_dead = TargetValue::new(0.0, 5.2);
        let tor_visible = TargetValue::new(0.0, 5.2);
        Self { alice, bob, carol, step, connections, https, connection_id, alice_dead, tor_visible }
    }
}
impl Scene for MotivationScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide("TOR - motivation", draw, frame);

        {
            let start = *self.carol.pos * 0.5;
            let end = lerp(start, vec2(0.0, -200.0), *self.connections.0);
            draw.line().start(start).end(end).weight(7.0).color(gray(0.5));
            draw.ellipse().xy(end).radius(10.0 * *self.connections.0).color(gray(0.5));
        }

        {
            let start = *self.carol.pos * 0.5;
            let end = lerp(start, *self.bob.pos, *self.connections.1);
            draw.line().start(start).end(end).weight(7.0).color(gray(0.5));
            let msg = lerp(start, end, (app.time * 1.2 + 0.24).sin() / 2.0 + 0.5);
            draw.ellipse().radius(start.distance(end).min(20.0)).xy(msg).color(DARKRED.with_alpha(0.95));
        }

        {
            let start = *self.alice.pos + vec2(200.0, 0.0);
            let end = *self.bob.pos - vec2(200.0, 0.0);

            draw.line().start(start).end(end).weight(12.0 * *self.bob.scale).color(gray(0.5));
            let msg = lerp(start, end, (app.time).sin() / 2.0 + 0.5);
            draw.ellipse().radius(start.distance(end).min(20.0) * *self.bob.scale).xy(msg).color(DARKRED.with_alpha(0.95));
            draw.line().start(start).end(end).weight(50.0 * *self.https).color(gray(0.4)).caps_round();

            {
                let text =  if self.step > 7 { "https://bobs-leaks.org" }
                            else { "https://bobs-shop.com" };
                draw.scale(*self.connection_id).text(text).font_size(30).color(gray(0.8)).y(-195.0).width(500.0);
            }

            draw.line().start(start).end(end).weight(100.0 * *self.tor_visible).color(gray(0.6)).caps_round();
            draw.scale(*self.tor_visible).text("< TOR >").font_size(40).color(gray(0.8)).y(-195.0).width(500.0);
        }


        let alice_box = self.alice.draw(draw).pad(-20.0);
        draw.line().start(alice_box.top_left()).end(alice_box.bottom_right()).color(gray(0.1)).weight(*self.alice_dead * 25.0);
        draw.line().start(alice_box.top_right()).end(alice_box.bottom_left()).color(gray(0.1)).weight(*self.alice_dead * 25.0);

        self.bob.draw(draw);
        self.carol.draw(draw);
    }

    fn update(&mut self, _app: &App, dt: Duration, _t: Duration) {
        let dt = dt.as_secs_f32();
        self.alice.update(dt);
        self.bob.update(dt);
        self.carol.update(dt);
        self.connections.0.update(dt);
        self.connections.1.update(dt);
        self.https.update(dt);
        self.connection_id.update(dt);
        self.alice_dead.update(dt);
        self.tor_visible.update(dt);
    }

    fn reset(&mut self) {
        *self = Self::new();
        // self.step = 0;
    }

    fn next_step(&mut self) -> NextStep {
        match self.step {
            0 => self.alice.scale.goto(1.0), // show alice
            1 => { // add bob
                self.alice.pos.goto(vec2(-400.0, -200.0));
                self.bob.scale.goto(1.0);
                self.bob.pos.goto(vec2(400.0, -200.0));
            },
            2 => { // add carol
                self.carol.pos.goto(vec2(0.0, 300.0));
                self.carol.scale.goto(1.0);
            },
            3 => { // carol engage spy
                self.connections.0.goto(1.0);
            },
            4 => { // carol do alice's transactions
                self.connections.1.goto(0.7);
            },
            5 => { // enable https
                self.https.goto(1.0);
                self.connections.1.goto(0.0);
                self.connections.0.goto(0.8);
            },
            6 => { // show addres
                self.connection_id.goto(1.0);
            },
            7 => {}, // change to bobs-leaks.org
            8 => { // kill alice
                self.alice_dead.goto(1.0);
            },
            9 => { // unkill alice
                self.alice_dead.goto(0.0);
            },
            10 => {
                self.alice.pos.goto(vec2(-480.0, -200.0));
                self.bob.pos.goto(vec2(480.0, -200.0));
                self.tor_visible.goto(1.0);
                self.connections.0.goto(0.0);
            },
            _ => { return NextStep::Finished },
        }
        self.step += 1;
        NextStep::Running
    }
}

struct NetNode {
    name: String,
    pos: TargetValue<Vec2>,
    scale: TargetValue<f32>,
}
impl NetNode {
    fn new(name: impl Into<String>, pos: impl Into<Vec2>) -> Self {
        let name = name.into();
        let pos = TargetValue::new(pos.into(), 5.2);
        let scale = TargetValue::new(0.0, 5.2);
        Self { name, pos, scale }
    }

    fn draw(&self, draw: &Draw) {
        draw.scale(*self.scale).ellipse().radius(100.0).xy(*self.pos).color(DARKRED.with_alpha(0.99));
        draw.scale(*self.scale).text(&self.name).font_size(40).xy(*self.pos).wh(vec2(100.0, 100.0)).color(gray(0.8));
    }

    fn update(&mut self, dt: f32) {
        self.pos.update(dt);
        self.scale.update(dt);
    }
}
struct BuildCircScene {
    alice: NetNode,
    bob: NetNode,
    ors: (NetNode, NetNode),
    message: TargetValue<Vec2>,
    msg: String,
    step: u32,
}

impl BuildCircScene {
    fn new() -> Self {
        let alice = NetNode::new("Alice", (-600.0, 0.0));
        let message = TargetValue::new(*alice.pos, 2.2);
        let bob = NetNode::new("Bob", (600.0, 0.0));
        let ors = (NetNode::new("OR1", (-200.0, 0.0)), NetNode::new("OR2", (200.0, 0.0)));
        let step = 0;
        let msg = String::new(); // String::from("create c1, <g, OR1_k>");
        Self {
            alice,
            bob,
            message,
            ors,
            msg,
            step,
        }
    }
}
impl Scene for BuildCircScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        let info_rect = Rect::from_w_h(frame.pad(10.0).w() * *self.alice.scale, 300.0).align_bottom_of(frame.pad(10.0));
        draw.line().start(info_rect.top_left()).end(info_rect.top_right()).weight(10.0).color(gray(0.05));
        draw.text(&self.msg).xy(info_rect.xy()).wh(info_rect.wh()).center_justify().font_size(30).color(gray(0.8).into_format().with_alpha(*self.alice.scale));

        draw_slide("TOR - implementation", draw, frame);

        {
            let draw = draw.translate(vec2(0.0, 150.0).extend(0.0));
            draw.scale(*self.alice.scale).ellipse().radius(20.0).color(DARKRED.with_alpha(0.3)).xy(*self.message);

            self.alice.draw(&draw);
            self.bob.draw(&draw);
            self.ors.0.draw(&draw);
            self.ors.1.draw(&draw);
        }
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {
        let dt = dt.as_secs_f32();

        self.alice.update(dt);
        self.bob.update(dt);
        self.ors.0.update(dt);
        self.ors.1.update(dt);
        self.message.update(dt);
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn next_step(&mut self) -> NextStep {
        match self.step {
            0 => {
                self.alice.scale.goto(1.0);
                self.bob.scale.goto(1.0);
            },
            1 => {
                self.ors.0.scale.goto(1.0);
                self.ors.1.scale.goto(1.0);
            },
            2 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("create c1, <g^x1, OR1_k>");
            },
            3 => {
                self.message.goto(*self.alice.pos);
                self.msg = String::from("created c1, g^y1, H(key_1)");
            },
            4 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("relay c1 { extend, OR2, <g^x2, OR2_k> }");
            },
            5 => {
                self.message.goto(*self.ors.1.pos);
                self.msg = String::from("create c2, <g^x2, OR2_k>");
            },
            6 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("created c2, g^y2, H(key_2)");
            },
            7 => {
                self.message.goto(*self.alice.pos);
                self.msg = String::from("relay c2 { extended, g^y2, H(key_2) }");
            },
            8 => {
                self.msg = String::from("-- circuit established --");
            },
            9 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("Relay c1 {{ begin \"https://bobs-leaks.org\" }}");
            },
            10 => {
                self.message.goto(*self.ors.1.pos);
                self.msg = String::from("Relay c2 { begin \"https://bobs-leaks.org\" }");
            },
            11 => {
                self.message.goto(*self.bob.pos);
                self.msg = String::from("-- handshake --");
            },
            12 => {
                self.message.goto(*self.ors.1.pos);
                self.msg = String::from("-- handshake --");
            }
            13 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("relay c2 { connected }");
            },
            14 => {
                self.message.goto(*self.alice.pos);
                self.msg = String::from("relay c1 {{ connected }}");
            },
            15 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("relay c1 {{ data, \"HTTP GET ...\" }}");
            },
            16 => {
                self.message.goto(*self.ors.1.pos);
                self.msg = String::from("relay c2 { data, \"HTTP GET ...\" }");
            },
            17 => {
                self.message.goto(*self.bob.pos);
                self.msg = String::from("\"HTTP GET\"");
            },
            18 => {
                self.message.goto(*self.ors.1.pos);
                self.msg = String::from("(response)");
            },
            19 => {
                self.message.goto(*self.ors.0.pos);
                self.msg = String::from("relay c2 { (response) }");
            },
            20 => {
                self.message.goto(*self.alice.pos);
                self.msg = String::from("relay c1 {{ (response) }}");
            },
            21 => {
                self.msg = String::from("-- website --");
            },
            _ => { return NextStep::Finished },
        }
        self.step += 1;
        NextStep::Running
    }
}

struct DisadvantageScene {

}
impl DisadvantageScene {
    fn new() -> Self {
        Self {

        }
    }
}
impl Scene for DisadvantageScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide("TOR - disadvantages", draw, frame);
        draw.text(r#"
            - no no-trust environment
            - unsecure against end-to-end attacks
            - can not conceal connection
        "#).w_h(1000.0, 1000.0).left_justify().font_size(50).color(gray(0.8));
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {

    }
}

struct FunctionalityScene {

}
impl FunctionalityScene {
    fn new() -> Self {
        Self {

        }
    }
}
impl Scene for FunctionalityScene {
    fn draw(&self, app: &App, draw: &Draw, frame: &Rect) {
        draw_slide("TOR - functionality", draw, frame);
        let frame = frame.pad(10.0);
        draw_title_block("symetric  -  asymetric", Rect::from_x_y_w_h(0.0, 400.0, 1000.0, 200.0), 40, draw);
        let left = Rect::from_w_h(frame.w() / 4.0, frame.h()).align_left_of(frame).shift_y(-200.0);
        let right = Rect::from_w_h(frame.w() / 4.0, frame.h()).align_right_of(frame).shift_y(-200.0);

        draw_title_block("Alice", Rect::from_x_y_w_h(-300.0, 100.0, 1000.0, 200.0), 30, draw);
        draw_title_block("Bob", Rect::from_x_y_w_h(-300.0, -300.0, 1000.0, 200.0), 30, draw);

        draw.line().start(vec2(-300.0, 0.0)).end(vec2(-300.0, -200.0)).weight(15.0).color(gray(0.5));
        draw.ellipse().radius(50.0).xy(vec2(-300.0, -100.0)).color(gray(0.06));
        draw.ellipse().radius(50.0).xy(vec2(-300.0, -100.0)).stroke(gray(0.5)).no_fill().stroke_weight(10.0);
        draw.text("shared\nsecret").wh(vec2(60.0, 60.0)).xy(vec2(-300.0, -100.0)).color(gray(0.8)).font_size(20);

        draw_title_block("Alice", Rect::from_x_y_w_h(300.0, 100.0, 1000.0, 200.0), 30, draw);
        draw_title_block("Bob", Rect::from_x_y_w_h(300.0, -300.0, 1000.0, 200.0), 30, draw);
        draw.text("public Keys").xy(vec2(300.0, -100.0)).font_size(20).color(gray(0.8));
        draw.text("Bob's private Key").xy(vec2(300.0, -400.0)).font_size(20).color(gray(0.6));
        draw.text("Alice's private Key").xy(vec2(300.0, 200.0)).font_size(20).color(gray(0.6));
    }

    fn update(&mut self, app: &App, dt: Duration, t: Duration) {

    }
}

struct Model {
    current: usize,
    scenes: Vec<Box<dyn Scene>>,
}

fn scene<S: Scene + 'static>(scene: S) -> Box<dyn Scene> { Box::new(scene) }
impl Model {
    fn new(app: &App) -> Self {
        app.new_window()
            .view(view)
            .key_pressed(key_pressed)
            .fullscreen()
        .build().expect("could not create window");

        let current = 0;
        let scenes = vec![
            scene(TitleScene::new()),
            scene(GeneralScene::new()),
            scene(TimelineScene::new(vec![
                TimelineEvent::new("mid 1990s", "US Naval Research Employees begin developement", None),
                TimelineEvent::new("2002", "alpha version is launched", None),
                TimelineEvent::new("2003", "first public release", None),
                TimelineEvent::new("2004", "code released under free license", None),
                TimelineEvent::new("2006", "'The Tor Project' non profit is founded", None),
                TimelineEvent::new("2007", "The Organisation begins deploying Bridges over Goverment firewalls", None),
                TimelineEvent::new("2008", "Development on the Tor Browser begins", None),
                TimelineEvent::new("2010", "Tor proves itself invaluable during Arab Spring", None),
                TimelineEvent::new("2013", "Tor helps Edward Snowden publish his documents on american surveillance", None),
            ], "TOR - history")),
            scene(FundingScene::new(app)),
            scene(QuoteScene::new(
                "What is Tor trying to achieve?",
                "",
                "TOR - motivation",
            )),
            scene(MotivationScene::new()), // alice and bob
            scene(FunctionalityScene::new()),
            scene(BuildCircScene::new()),
            scene(QuoteScene::new(
                "Tor Browser aims to make all users look the same, making if difficult for you to be fingerprinted on your browser and device information",
                "- The Tor Browser GitLab",
                "TOR - the browser",
            )),
            scene(DisadvantageScene::new()),

            // functionality scenes
            scene(QuoteScene::new(
                "usability is not only a convinience: it is a security requirement",
                "- the TOR whitepaper",
                "TOR - conclusion",
            )),
        ];

        Self { scenes, current }
    }

    fn current_scene(&self) -> &Box<dyn Scene> { &self.scenes[self.current] }
    fn current_mut(&mut self) -> &mut Box<dyn Scene> { &mut self.scenes[self.current] }

    fn next_scene(&mut self) {
        self.current_mut().reset();
        self.current = (self.current + 1) % self.scenes.len();
    }

    fn next_step(&mut self) {
        if self.current_mut().next_step().is_finished() {
            self.next_scene();
        }
    }

    fn prev_scene(&mut self) {
        self.current_mut().reset();
        if self.current == 0 {
            self.current = self.scenes.len() - 1;
        } else { self.current = self.current - 1; }
    }

    fn event(&mut self, app: &App, event: Event) {
        match event {
            Event::Update(Update { since_last, since_start }) => {
                if self.scenes.len() > 0 {
                    self.current_mut().update(app, since_last, since_start);
                }
            },
            _ => {},
        }
    }
    
    fn view(&self, app: &App, frame: Frame) {
        let scale = frame.rect().w().max(frame.rect().h()) / 1920.0;
        let draw = app.draw().scale(scale);
        let rect = Rect::from_xy_wh(frame.rect().xy() / scale, frame.rect().wh() / scale);

        {
            let r = rect.pad(10.0);
            draw.rect().wh(r.wh()).xy(rect.xy()).color(gray(0.06)).z(-100.0);
        }

        if self.scenes.len() > 0 {
            self.current_scene().draw(app, &draw, &rect);
        } else { draw.text("no scene").font_size(50); }

        #[cfg(debug_assertions)]
        draw.text(&format!("fps: {:.02}", app.fps())).xy(rect.top_left() + vec2(70.0, -30.0)).font_size(20);
        draw.background().color(gray(0.02));
        draw.to_frame(app, &frame).expect("could not draw frame");
    }

    fn key_pressed(&mut self, app: &App, key: Key) {
        match key {
            Key::Right | Key::Space => self.next_step(),
            Key::Left | Key::Back => self.prev_scene(),
            _ => {},
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) { model.view(app, frame) }
fn event(app: &App, model: &mut Model, event: Event) { model.event(app, event) }
fn key_pressed(app: &App, model: &mut Model, key: Key) { model.key_pressed(app, key) }

fn main() {
    nannou::app(Model::new)
        .event(event)
    .run();
}


