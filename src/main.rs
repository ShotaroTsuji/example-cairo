use cairo::*;

fn draw_example(context: &Context) {
    context.move_to(100.0, 100.0);
    context.line_to(100.0, 150.0);
    context.line_to(150.0, 100.0);
    context.stroke();

    context.move_to(300.0, 300.0);
    context.rel_line_to(-100.0, 0.0);
    context.rel_line_to(0.0, -100.0);
    context.rel_line_to(100.0, 100.0);
    context.close_path();
    context.stroke();

    context.set_dash(&[5.0, 2.5], 0.0);
    context.set_line_width(5.0);
    context.move_to(100.0, 400.0);
    context.rel_curve_to(10.0, 10.0,
                         50.0, 20.0,
                         100.0, 0.0);
    context.rel_curve_to(50.0, -20.0,
                         50.0, -60.0,
                         100.0, 0.0);
    context.stroke();

    let pat = SolidPattern::from_rgb(0.0, 1.0, 0.5);
    context.set_source(&pat);
    context.rectangle(500.0, 100.0, 50.0, 80.0);
    context.fill();

    context.set_font_size(30.0);
    let ext = context.text_extents("Hello, world!");
    let grad = LinearGradient::new(400.0, 400.0,
                                   400.0+ext.width, 400.0+ext.height);
    grad.add_color_stop_rgb(0.0, 1.0, 0.0, 0.0);
    grad.add_color_stop_rgb(0.5, 0.8, 0.8, 0.0);
    grad.add_color_stop_rgb(1.0, 0.0, 1.0, 0.0);
    context.set_source(&grad);
    context.move_to(400.0, 400.0);
    context.show_text("Hello, world!");
}

fn main() {
    let surface = PdfSurface::new(600.0, 600.0, "test.pdf");
    let context = Context::new(&surface);
 
    draw_example(&context);
}
