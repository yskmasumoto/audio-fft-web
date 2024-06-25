use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_fft_v2(samples: &[f32]) -> Vec<f32> {
    // サンプル数
    let n = samples.len();

    // フーリエ変換のためのバッファを準備
    let mut input: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();

    // FFTを計算
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut input);

    // パワースペクトルを計算
    let power_spectrum: Vec<f32> = input.iter().map(|c| c.norm_sqr()).collect();

    // パワースペクトルをデシベルに変換
    let power_spectrum_db: Vec<f32> = power_spectrum.iter()
        .map(|&power| 10.0 * f32::log10(power))
        .collect();

    return power_spectrum_db;
}
