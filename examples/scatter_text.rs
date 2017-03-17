extern crate plotlib;

fn main() {
    let data = [(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    let s1 = plotlib::scatter::Scatter::from_vec(&data);
    let s2 = plotlib::scatter::Scatter::from_vec(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(plotlib::scatter::Style::new().marker(plotlib::scatter::Marker::Square));
    let v = plotlib::view::View::new()
        .add(&s1)
        .add(&s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The repsonse of something");
    println!("{}", plotlib::plot::Plot::single(&v).to_text());
}
