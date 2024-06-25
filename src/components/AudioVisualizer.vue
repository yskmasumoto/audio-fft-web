<script setup>
import { ref, onMounted } from 'vue';
import init, { calculate_fft_v2 } from '../rust_wasm/pkg/audio_fft.js';

const props = defineProps({
  audioSrc: String
});

const audioElement = ref(null);
const spectrumCanvas = ref(null);
let audioContext = null;
let analyserNode = null;
let dataArray = null;
let canvasContext = null;
let isWasmLoaded = false;

onMounted(async () => {
  canvasContext = spectrumCanvas.value.getContext('2d');
  await init();
  isWasmLoaded = true;
});

function startAudioContext() {
  if (!audioContext) {
    audioContext = new AudioContext();
    analyserNode = audioContext.createAnalyser();

    const source = audioContext.createMediaElementSource(audioElement.value);
    source.connect(analyserNode);
    analyserNode.connect(audioContext.destination);

    dataArray = new Float32Array(analyserNode.fftSize);
  }

  requestAnimationFrame(updateSpectrum);
}

function updateSpectrum() {
  if (!isWasmLoaded) {
    requestAnimationFrame(updateSpectrum);
    return;
  }

  analyserNode.getFloatTimeDomainData(dataArray);

  // WebAssemblyを使ってFFTを計算
  const powerSpectrumDb = calculate_fft_v2(dataArray);

  canvasContext.clearRect(0, 0, spectrumCanvas.value.width, spectrumCanvas.value.height);

  const barWidth = spectrumCanvas.value.width / powerSpectrumDb.length;
  let barHeight;
  let x = 0;

  for (let i = 0; i < powerSpectrumDb.length; i++) {
    barHeight = powerSpectrumDb[i] + 100;

    canvasContext.fillStyle = 'rgb(' + (barHeight + 50) + ',50,50)';
    canvasContext.fillRect(x, spectrumCanvas.value.height - barHeight / 2, barWidth, barHeight / 2);

    x += barWidth + 1;
  }

  requestAnimationFrame(updateSpectrum);
}
</script>

<template>
  <div>
    <audio ref="audioElement" :src="audioSrc" controls></audio>
    <canvas 
      ref="spectrumCanvas"
      width="800"
      height="600"
    ></canvas>
    <button @click="startAudioContext">Start Visualization</button>
  </div>
</template>