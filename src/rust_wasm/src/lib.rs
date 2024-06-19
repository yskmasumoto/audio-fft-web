use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_fft(audio_data: &[f32]) -> Vec<f32> {
    // FFTの準備
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(audio_data.len());

    // 入力データを複素数の配列に変換
    let mut buffer: Vec<Complex<f32>> = audio_data.iter().map(|&x| Complex::new(x, 0.0)).collect();
    
    // FFTを実行
    fft.process(&mut buffer);

    // 結果の実部と虚部の大きさを計算してベクトルに格納
    buffer.iter().map(|c| c.norm()).collect()
}
