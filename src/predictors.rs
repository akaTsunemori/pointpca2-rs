use std::f64::EPSILON;

use na::{DMatrix, MatrixView};

fn relative_difference(x: f64, y: f64) -> f64 {
    return 1. - (x - y).abs() / (x.abs() + y.abs() + EPSILON);
}

fn iter_relative_difference<'a, T: na::Dim>(
    x: &'a MatrixView<f64, T, T>,
    y: &'a MatrixView<f64, T, T>,
) -> DMatrix<f64> {
    assert_eq!(x.nrows(), y.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), y.ncols(), "Matrices must have the same shape.");
    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut result = DMatrix::zeros(nrows, ncols);
    for i in 0..nrows {
        for j in 0..ncols {
            result[(i, j)] = relative_difference(x[(i, j)], y[(i, j)]);
        }
    }
    result
}

fn textural_covariance<'a, T: na::Dim>(
    x: &'a MatrixView<f64, T, T>,
    y: &'a MatrixView<f64, T, T>,
    z: &'a MatrixView<f64, T, T>,
) -> DMatrix<f64> {
    assert_eq!(x.nrows(), y.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), y.ncols(), "Matrices must have the same shape.");
    assert_eq!(x.nrows(), z.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), z.ncols(), "Matrices must have the same shape.");
    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut result = DMatrix::zeros(nrows, ncols);
    let mut x_ij: f64;
    let mut y_ij: f64;
    let mut z_ij: f64;
    for i in 0..nrows {
        for j in 0..ncols {
            x_ij = x[(i, j)];
            y_ij = y[(i, j)];
            z_ij = z[(i, j)];
            result[(i, j)] =
                (x_ij.sqrt() * y_ij.sqrt() - z_ij).abs() / (x_ij.sqrt() * y_ij.sqrt() + EPSILON);
        }
    }
    result
}

fn textural_variance_sum<'a, T: na::Dim>(
    x: &'a MatrixView<f64, T, T>,
    y: &'a MatrixView<f64, T, T>,
) -> DMatrix<f64> {
    assert_eq!(x.nrows(), y.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), y.ncols(), "Matrices must have the same shape.");
    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut result = DMatrix::zeros(nrows, 1);
    for i in 0..nrows {
        let mut x_sum: f64 = 0.;
        let mut y_sum: f64 = 0.;
        for j in 0..ncols {
            x_sum += x[(i, j)];
            y_sum += y[(i, j)];
        }
        result[(i, 0)] = relative_difference(x_sum, y_sum);
    }
    result
}

fn textural_omnivariance<'a, T: na::Dim>(
    x: &'a MatrixView<f64, T, T>,
    y: &'a MatrixView<f64, T, T>,
) -> DMatrix<f64> {
    assert_eq!(x.nrows(), y.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), y.ncols(), "Matrices must have the same shape.");
    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut result = DMatrix::zeros(nrows, 1);
    for i in 0..nrows {
        let mut x_prod: f64 = 1.;
        let mut y_prod: f64 = 1.;
        for j in 0..ncols {
            x_prod *= x[(i, j)];
            y_prod *= y[(i, j)];
        }
        result[(i, 0)] = relative_difference(x_prod.cbrt(), y_prod.cbrt());
    }
    result
}

fn textural_entropy<'a, T: na::Dim>(
    x: &'a MatrixView<f64, T, T>,
    y: &'a MatrixView<f64, T, T>,
) -> DMatrix<f64> {
    assert_eq!(x.nrows(), y.nrows(), "Matrices must have the same shape.");
    assert_eq!(x.ncols(), y.ncols(), "Matrices must have the same shape.");
    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut result = DMatrix::zeros(nrows, 1);
    for i in 0..nrows {
        let mut x_entropy: f64 = 0.;
        let mut y_entropy: f64 = 0.;
        for j in 0..ncols {
            x_entropy += x[(i, j)] * (x[(i, j)] + EPSILON).ln();
            y_entropy += y[(i, j)] * (y[(i, j)] + EPSILON).ln();
        }
        result[(i, 0)] = relative_difference(x_entropy, y_entropy);
    }
    result
}

pub fn compute_predictors<'a>(local_features: &'a DMatrix<f64>) -> DMatrix<f64> {
    let nrows = local_features.nrows();
    let ncols = local_features.ncols();
    let projection_a_to_a = local_features.view((0, 0), (nrows, 3));
    let projection_b_to_a = local_features.view((0, 3), (nrows, 3));
    let colors_mean_a = local_features.view((0, 6), (nrows, 3));
    let points_mean_b = local_features.view((0, 9), (nrows, 3));
    let colors_mean_b = local_features.view((0, 12), (nrows, 3));
    let points_variance_a = local_features.view((0, 15), (nrows, 3));
    let colors_variance_a = local_features.view((0, 18), (nrows, 3));
    let points_variance_b = local_features.view((0, 21), (nrows, 3));
    let colors_variance_b = local_features.view((0, 24), (nrows, 3));
    let points_covariance_ab = local_features.view((0, 27), (nrows, 3));
    let colors_covariance_ab = local_features.view((0, 30), (nrows, 3));
    let points_eigenvectors_b_x = local_features.view((0, 33), (nrows, 3));
    let points_eigenvectors_b_y = local_features.view((0, 36), (nrows, 3));
    let points_eigenvectors_b_z = local_features.view((0, 39), (nrows, 3));
    let mut predictors = DMatrix::zeros(nrows, 40);
    predictors.fill(f64::NAN);

    // Textural predictors
    predictors
        .view_mut((0, 0), (nrows, 3))
        .copy_from(&iter_relative_difference(&colors_mean_a, &colors_mean_b));
    predictors
        .view_mut((0, 3), (nrows, 3))
        .copy_from(&iter_relative_difference(
            &colors_variance_a,
            &colors_variance_b,
        ));
    predictors
        .view_mut((0, 6), (nrows, 3))
        .copy_from(&textural_covariance(
            &colors_variance_a,
            &colors_variance_b,
            &colors_covariance_ab,
        ));
    predictors
        .view_mut((0, 9), (nrows, 1))
        .copy_from(&textural_variance_sum(
            &colors_variance_a,
            &colors_variance_b,
        ));
    predictors
        .view_mut((0, 10), (nrows, 1))
        .copy_from(&textural_omnivariance(
            &colors_variance_a,
            &colors_variance_b,
        ));
    predictors
        .view_mut((0, 10), (nrows, 1))
        .copy_from(&textural_entropy(&colors_variance_a, &colors_variance_b));
    // Geometric predictors
    // To-Do
    // . . .

    predictors
}
