use plotters::prelude::*;
use std::fmt::Write;

fn plotty(path: &str, data: &[i32]) {
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(100)
        .y_label_area_size(100)
        .build_cartesian_2d(
            0.0..(data.len() as f64),
            //-1_000_000_000.0..2_000_000_000.0_f64,
            -1_000_000.0..2_000_000.0_f64,
        )
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..data.len()).map(|x| {
                println!("{}", data[x]);
                (x as f64, data[x] as f64)
            }),
            &RED,
        ))
        .unwrap();

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}

pub fn part1(lines: &[&str], out: &mut String) {
    let crabs: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let max_coord = crabs.iter().fold(0i32, |a, x| a.max(*x));

    let fuels: Vec<i32> = (0..max_coord)
        .map(|i| crabs.iter().map(|x| (x - i as i32).abs()).sum::<i32>())
        .collect();

    plotty("1.png", &fuels);

    //    for i in 0..(fuels.len() - 1) {
    //        println!("{}", fuels[i + 1] - fuels[i]);
    //    }

    //let min_fuel = fuels.fold(std::i32::MAX, |a, x| a.min(x));

    let result = 0;
    write!(out, "{}", result).unwrap();
}

pub fn part2(lines: &[&str], out: &mut String) {
    let crabs: Vec<i32> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let max_coord = crabs.iter().fold(0i32, |a, x| a.max(*x));

    let fuels: Vec<i32> = (0..max_coord)
        .map(|i| {
            crabs
                .iter()
                .map(|x| {
                    let delta = (x - i as i32).abs();
                    delta * (delta + 1) / 2
                })
                .sum::<i32>()
        })
        .collect();

    plotty("2.png", &fuels);
    //let min_fuel = fuels.fold(std::i32::MAX, |a, x| a.min(x));

    let result = 0; //min_fuel;
    write!(out, "{}", result).unwrap();
}
