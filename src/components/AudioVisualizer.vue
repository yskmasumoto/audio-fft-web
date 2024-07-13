<script setup>
import { ref, onMounted } from 'vue';
import init, { calculate_fft_v2 } from '../rust_wasm/pkg/audio_fft.js';
import FileUploadComponent from './FileUpload.vue'
import AudioPlayerComponent from './AudioPlayer.vue'
import AudioMeterComponent from './AudioMeter.vue'

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
const maxdb = ref(0);
const meandb = ref(0);
const aryMax = function (a, b) {return Math.max(a, b);}
const playflag = ref(false)

const update_isPlaying = (isPlaying) => {
  console.log('Received isPlaying:', isPlaying);
  playflag.value = isPlaying
}

onMounted(async () => {
  canvasContext = spectrumCanvas.value.getContext('2d');
  await init();
  isWasmLoaded = true;
});

const handleFileUpload = async (file) => {
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
  let powerSpectrumDb = calculate_fft_v2(dataArray);
  powerSpectrumDb = powerSpectrumDb.slice(0, powerSpectrumDb.length / 2)

  canvasContext.clearRect(0, 0, spectrumCanvas.value.width, spectrumCanvas.value.height);

  const barWidth = spectrumCanvas.value.width / powerSpectrumDb.length;
  let barHeight;
  let x = 0;
  let fill_style = 'rgb(20,20,20)'
  let sum = 0;

  for (let i = 0; i < powerSpectrumDb.length; i++) {
    barHeight = powerSpectrumDb[i] * 4;
    if (barHeight < 0) {
      fill_style = 'rgb(0,' + (-barHeight + 50) + ',0)';
    } else {
      fill_style = 'rgb(' + (barHeight + 50) + ',0,0)';
    }
    sum += powerSpectrumDb[i]
    canvasContext.fillStyle = fill_style
    canvasContext.fillRect(x, spectrumCanvas.value.height / 2 - barHeight / 2, barWidth, barHeight / 2);

    x += barWidth;
  }

  if (maxdb.value < powerSpectrumDb.reduce(aryMax)){
    maxdb.value = powerSpectrumDb.reduce(aryMax)
  }
  meandb.value = sum / powerSpectrumDb.length

  requestAnimationFrame(updateSpectrum);
}
</script>

<template>
  <div class="audiovisualizer">
    <div class="uploader">
        <FileUploadComponent @file-uploaded="handleFileUpload" />
        <AudioPlayerComponent @change-play-pause="update_isPlaying"/>
    </div>
    <div class="spectrumarea">
      <canvas 
        ref="spectrumCanvas"
        width="800"
        height="600"
      ></canvas>
    </div>
    <AudioMeterComponent :peakLevel="maxdb" :is_playing="playflag" />
  </div>  
</template>

<style scoped>
canvas {
  border: 5px solid black;
  background-color: rgb(150, 150, 150);
}

.audiovisualizer {
  margin: auto;
  width: 1000px;
}

.spectrumarea {
  display: flex;
  justify-content: center;
}

.uploader {
  margin-left: 175px;
  margin-right: 175px;
  height: 50px;
  justify-content: space-around;
  align-items: center;
  display: flex;
  flex: 1 2 auto;
}
</style>