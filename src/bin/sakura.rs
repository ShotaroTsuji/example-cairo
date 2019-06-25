/// Generates an image of scattered cherry blossom petals
use cairo::*;
use rand::distributions::{Uniform, Distribution};

const SIZE: f64 = 600.0;

fn make_sakura(context: &Context) -> Path {
    context.save();

    context.set_line_cap(LineCap::Round);
    context.set_line_join(LineJoin::Round);
    context.move_to(0.0, 0.0);
    context.rel_curve_to(20.0, -20.0,
                         20.0, -45.0,
                         10.0, -60.0);
    context.rel_curve_to(-3.0, 1.0,
                         -9.0, 7.0,
                         -10.0, 10.0);
    context.rel_curve_to(-1.0, -3.0,
                         -7.0, -9.0,
                         -10.0, -10.0);
    context.rel_curve_to(-10.0, 15.0,
                         -10.0, 40.0,
                         10.0, 60.0);
    context.close_path();

    let path = context.copy_path();

    context.restore();

    path
}

fn draw_sakura(context: &Context) {
    let sakura = make_sakura(context);

    let pos = Uniform::new(0.0, SIZE);
    let rot = Uniform::new(0.0, 360.0);
    let scl = Uniform::new(0.25, 1.0);
    let mut rng = rand::thread_rng();

    context.set_source_rgb(1.0, 0.8, 0.8);

    for _ in 0..100 {
        context.save();

        context.new_path();
        context.translate(pos.sample(&mut rng), pos.sample(&mut rng));
        context.rotate(rot.sample(&mut rng));
        let k = scl.sample(&mut rng);
        context.scale(k, k);
        context.append_path(&sakura);
        context.fill();

        context.restore();
    }
}

fn main() {
    let surface = PdfSurface::new(SIZE, SIZE, "sakura.pdf");
    let context = Context::new(&surface);
 
    draw_sakura(&context);
}
