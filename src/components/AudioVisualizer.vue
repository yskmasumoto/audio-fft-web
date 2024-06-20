<script setup lang="ts">
import { ref, onMounted } from "vue";
import init, {
	calculate_fft,
	prepare_fft_data,
} from "../rust_wasm/pkg/audio_fft.js";

const audioContext = ref<AudioContext | null>(null);
const analyser = ref<AnalyserNode | null>(null);
const dataArray = ref<Float32Array | null>(null);
const canvas = ref<HTMLCanvasElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);
const audioElement = ref<HTMLAudioElement | null>(null);
const sourceNode = ref<MediaElementAudioSourceNode | null>(null);
let scaleFactor: number = 0.001;
let maxFrequency: number = 2000;

onMounted(async () => {
	await init();
	audioContext.value = new (window.AudioContext || window.webkitAudioContext)();
	analyser.value = audioContext.value.createAnalyser();
	analyser.value.fftSize = 16384;
	const bufferLength = analyser.value.frequencyBinCount;
	dataArray.value = new Float32Array(bufferLength);

	canvas.value = document.getElementById("canvas") as HTMLCanvasElement;
	ctx.value = canvas.value.getContext("2d") as CanvasRenderingContext2D;

	audioElement.value = document.getElementById("audio") as HTMLAudioElement;
	sourceNode.value = audioContext.value.createMediaElementSource(
		audioElement.value,
	);
	sourceNode.value.connect(analyser.value);
	analyser.value.connect(audioContext.value.destination);

	console.log(`Sample Rate: ${audioContext.value.sampleRate} Hz`);

	audioElement.value.addEventListener("canplay", () => {
		audioElement.value?.play();
	});

	draw();
});

const handleFileUpload = async (event: Event) => {
	if (audioContext.value?.state === "suspended") {
		await audioContext.value.resume();
	}

	const target = event.target as HTMLInputElement;
	const file = target.files?.[0];
	if (!file) return;
	const reader = new FileReader();

	reader.onload = (e) => {
		audioElement.value!.src = e.target!.result as string;
	};

	reader.readAsDataURL(file);
};

const draw = () => {
	requestAnimationFrame(draw);

	if (!analyser.value || !dataArray.value || !ctx.value || !canvas.value)
		return;

	analyser.value.getFloatFrequencyData(dataArray.value);

	const fftResult = calculate_fft(dataArray.value);
	const preparedData = prepare_fft_data(
		fftResult,
		audioContext.value!.sampleRate,
		analyser.value!.fftSize,
		maxFrequency,
		canvas.value.height,
		scaleFactor,
	);

	ctx.value.fillStyle = "rgb(0, 0, 0)";
	ctx.value.fillRect(0, 0, canvas.value.width, canvas.value.height);

	const binWidth = audioContext.value!.sampleRate / analyser.value!.fftSize;
	const maxIndex = Math.floor(maxFrequency / binWidth);

	const barWidth = (canvas.value.width / maxIndex) * 2.5;
	let x = 0;
	

	for (let i = 1; i < maxIndex; i++) {
		const barHeight = preparedData[i];
		ctx.value.fillStyle = `rgb(${barHeight},50,50)`;
		ctx.value.fillRect(x, canvas.value.height - barHeight, barWidth, barHeight);
		x += barWidth + 1;
	}
};
</script>

<template>
  <div class="spectolarea">
    <input class="spectolarea-elem" type="file" id="upload" @change="handleFileUpload" />
    <audio class="spectolarea-elem" id="audio" controls></audio>
    <canvas class="spectolarea-elem" id="canvas" width="1200" height="600"></canvas>
  </div>
</template>

<style scoped>
.spectolarea-elem {
  margin: auto;
}
.spectolarea {
  display: flex;
  flex-direction: column;
  text-align: center;
  justify-content: space-between;
  height: 800px;
  width: 100%;
  background-color: #222222;
}
audio {
  width: 800px;
}
</style>