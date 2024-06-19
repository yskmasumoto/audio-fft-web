<script setup lang="ts">
import { ref, onMounted } from "vue";
import init, { calculate_fft } from "../rust_wasm/pkg/audio_fft.js";

const audioContext = ref<AudioContext | null>(null);
const analyser = ref<AnalyserNode | null>(null);
const dataArray = ref<Float32Array | null>(null);
const canvas = ref<HTMLCanvasElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);
const audioElement = ref<HTMLAudioElement | null>(null);
const sourceNode = ref<MediaElementAudioSourceNode | null>(null);
const scaleFactor = 0.2;

onMounted(async () => {
	await init();
	audioContext.value = new (window.AudioContext || window.webkitAudioContext)();
	analyser.value = audioContext.value.createAnalyser();
	analyser.value.fftSize = 2048;
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

	const fftResult = calculate_fft(dataArray.value).map((v) => (v + 140) / 140);

	ctx.value.fillStyle = "rgb(0, 0, 0)";
	ctx.value.fillRect(0, 0, canvas.value.width, canvas.value.height);

	const barWidth = (canvas.value.width / dataArray.value.length) * 2.5;
	let barHeight;
	let x = 0;

	for (let i = 0; i < dataArray.value.length; i++) {
		barHeight = fftResult[i] * canvas.value.height * scaleFactor;

		const r = clamp(barHeight, 0, 255);
		const g = 50;
		const b = 50;
		ctx.value.fillStyle = "rgb(" + r + "," + g + "," + b + ")";

		ctx.value.fillRect(
			x,
			canvas.value.height - barHeight / 2,
			barWidth,
			barHeight / 2,
		);

		x += barWidth + 1;
	}
};

const clamp = (value: number, min: number, max: number) => {
	return Math.min(Math.max(value, min), max);
};
</script>

<template>
    <div>
      <canvas id="canvas" width="800" height="400"></canvas>
      <input type="file" id="upload" @change="handleFileUpload" />
      <audio id="audio" controls></audio>
    </div>
</template>
  
<style scoped>
canvas {
    border: 1px solid black;
}
</style>
  