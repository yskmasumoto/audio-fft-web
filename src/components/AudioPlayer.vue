<template>
  <div class="audio-player">
    <audio ref="audioElement" @loadedmetadata="onLoadedMetadata" @timeupdate="onTimeUpdate" @ended="onEnded">
      <source />
      Your browser does not support the audio element.
    </audio>
    <div class="controls">
      <a @click="togglePlayback">{{ isPlaying ? 'Pause' : 'Play' }}</a>
      <input type="range" :value="currentTime" :max="duration" @input="seekTo" class="seek-bar" :disabled="!loaded" />
      <span class="time">{{ formatTime(currentTime) }} / {{ formatTime(duration) }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const audioElement = ref(null);
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const loaded = ref(false);

onMounted(() => {
  audioElement.value.setAttribute('id', 'audio');
});

function togglePlayback() {
  if (isPlaying.value) {
    audioElement.value.pause();
  } else {
    audioElement.value.play();
  }
  isPlaying.value = !isPlaying.value;
}

function onLoadedMetadata() {
  duration.value = audioElement.value.duration;
  loaded.value = true;
}

function onTimeUpdate() {
  currentTime.value = audioElement.value.currentTime;
}

function seekTo(event) {
  audioElement.value.currentTime = event.target.value;
}

function onEnded() {
  isPlaying.value = false;
  currentTime.value = 0;
}

function formatTime(time) {
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
}
</script>

<style scoped>
/* ... */
</style>