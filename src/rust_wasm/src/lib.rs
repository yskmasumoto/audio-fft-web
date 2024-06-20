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

#[wasm_bindgen]
pub fn prepare_fft_data(
    fft_result: &[f32],
    sample_rate: f32,
    fft_size: usize,
    max_frequency: f32,
    canvas_height: f32,
    scale_factor: f32
) -> Vec<f32> {  // u8からf32に変更
    let bin_width = sample_rate / fft_size as f32;
    let max_index = (max_frequency / bin_width).floor() as usize;

    fft_result[1..max_index.min(fft_result.len())]
        .iter()
        .map(|&v| {
            let normalized = (v + 140.0) / 140.0;
            normalized * canvas_height * scale_factor
        })
        .collect()
}