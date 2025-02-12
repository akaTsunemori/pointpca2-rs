use crate::utils;
use na::DMatrix;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

fn column_axis_mean(
    vector: &Vec<(OrderedFloat<f64>, OrderedFloat<f64>, OrderedFloat<f64>)>,
) -> (f64, f64, f64) {
    let vec_len = vector.len() as f64;
    let mut vec_sum = (0., 0., 0.);
    for (a, b, c) in vector {
        vec_sum.0 += utils::from_ordered(*a);
        vec_sum.1 += utils::from_ordered(*b);
        vec_sum.2 += utils::from_ordered(*c);
    }
    let mean = (
        vec_sum.0 / vec_len,
        vec_sum.1 / vec_len,
        vec_sum.2 / vec_len,
    );
    mean
}

pub fn duplicate_merging(points: DMatrix<f64>, colors: DMatrix<u8>) -> (DMatrix<f64>, DMatrix<u8>) {
    let mut points_map: BTreeMap<
        (OrderedFloat<f64>, OrderedFloat<f64>, OrderedFloat<f64>),
        Vec<(OrderedFloat<f64>, OrderedFloat<f64>, OrderedFloat<f64>)>,
    > = BTreeMap::new();
    for i in 0..points.nrows() {
        let point = points.row(i);
        let point = (
            utils::to_ordered(point[0]),
            utils::to_ordered(point[1]),
            utils::to_ordered(point[2]),
        );
        let color = colors.row(i);
        let color = (
            utils::to_ordered(color[0] as f64),
            utils::to_ordered(color[1] as f64),
            utils::to_ordered(color[2] as f64),
        );
        points_map.entry(point).or_insert_with(Vec::new).push(color);
    }
    let nrows = points_map.len();
    let mut points_result = DMatrix::zeros(nrows, 3);
    let mut colors_result = DMatrix::zeros(nrows, 3);
    for (i, &key) in points_map.keys().enumerate() {
        let point = (
            utils::from_ordered(key.0),
            utils::from_ordered(key.1),
            utils::from_ordered(key.2),
        );
        let colors = points_map.get(&key).unwrap();
        let colors_mean = column_axis_mean(colors);
        points_result[(i, 0)] = point.0;
        points_result[(i, 1)] = point.1;
        points_result[(i, 2)] = point.2;
        colors_result[(i, 0)] = colors_mean.0.round() as u8;
        colors_result[(i, 1)] = colors_mean.1.round() as u8;
        colors_result[(i, 2)] = colors_mean.2.round() as u8;
    }
    (points_result, colors_result)
}

fn rgb_to_yuv(rgb: DMatrix<u8>) -> DMatrix<u8> {
    let rows = rgb.nrows();
    let rgb_f64 = rgb.map(|val| val as f64);
    let r = rgb_f64.column(0);
    let g = rgb_f64.column(1);
    let b = rgb_f64.column(2);
    let c = [
        0.2126, 0.7152, 0.0722, -0.1146, -0.3854, 0.5000, 0.5000, -0.4542, -0.0468,
    ];
    let o = [0., 128., 128.];
    let y = (c[0] * &r + c[1] * &g + c[2] * &b).add_scalar(o[0]);
    let u = (c[3] * &r + c[4] * &g + c[5] * &b).add_scalar(o[1]);
    let v = (c[6] * &r + c[7] * &g + c[8] * &b).add_scalar(o[2]);
    let mut yuv = DMatrix::zeros(rows, 3);
    for i in 0..rows {
        yuv[(i, 0)] = y[i].round() as u8;
        yuv[(i, 1)] = u[i].round() as u8;
        yuv[(i, 2)] = v[i].round() as u8;
    }
    yuv
}

pub fn preprocess_point_cloud(
    points: DMatrix<f64>,
    colors: DMatrix<u8>,
) -> (DMatrix<f64>, DMatrix<u8>) {
    let (points, colors) = duplicate_merging(points, colors);
    let colors = rgb_to_yuv(colors);
    (points, colors)
}
