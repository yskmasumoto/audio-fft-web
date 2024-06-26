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

const handleFileUpload = async (event) => {
	const target = event.target;
	const file = target.files?.[0];
	if (!file) return;
	const reader = new FileReader();
	reader.onload = (e) => {
		audioElement.value.src = e.target.result;
	};
	await reader.readAsDataURL(file);
  startAudioContext()
};

function startAudioContext() {
  console.log(audioElement)
  if (!audioContext) {
    audioElement.value = document.getElementById("audio")
    audioContext = new AudioContext();
    analyserNode = audioContext.createAnalyser();
    const source = audioContext.createMediaElementSource(audioElement.value);
    source.connect(analyserNode);
    analyserNode.connect(audioContext.destination);
    analyserNode.fftSize = 4096
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
  console.log(barWidth)
  for (let i = 0; i < powerSpectrumDb.length; i++) {
    barHeight = powerSpectrumDb[i];

    canvasContext.fillStyle = 'rgb(' + (barHeight + 50) + ',0,0)';
    canvasContext.fillRect(x, spectrumCanvas.value.height - barHeight / 2, barWidth, barHeight / 2);

    x += barWidth;
  }

  requestAnimationFrame(updateSpectrum);
}
</script>

<template>
  <div>
    <input type="file" id="upload" @change="handleFileUpload" />
    <audio id="audio" controls></audio>
    <canvas 
      ref="spectrumCanvas"
      width="800"
      height="600"
    ></canvas>
    <!-- <button @click="startAudioContext">Start Visualization</button> -->
  </div>  
</template>

<style scoped>
canvas {
  border: 2px solid black;
}
</style>