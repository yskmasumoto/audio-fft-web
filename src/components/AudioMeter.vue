<template>
<div class="audio-meter">
    <div class="meter">
    <img src="../assets/meter.svg" alt="metersvg">
    <div class="needle" :style="{ transform: `rotate(${vuRotation}deg)` }"></div>
    </div>
</div>
</template>

<script setup>
import { computed } from 'vue';

let viewdeg = 0
const maxDegree = 120; // degree
const maxLevel = 100; // db

const props = defineProps({
    peakLevel: {
    type: Number,
    required: true
    },
    is_playing: {
    type: Boolean,
    required: true
    }
});


const vuRotation = computed(() => {
    // Map the average level to rotation degrees
    console.log("meter component:", props.is_playing)
    if (!props.is_playing) {
        return 0
    }
    viewdeg = props.peakLevel * (maxDegree / maxLevel)
    return viewdeg
});

</script>
  
<style scoped>
.audio-meter {
    width: 300px;
    background-color: #1e1e1e;
    padding: 10px;
    border-radius: 5px;
    height: 300px;
}

.meter {
    position: relative;
    background-color: #000;
    margin-bottom: 10px;
    overflow: hidden;
}

.needle {
    position: absolute;
    bottom: 0;
    left: 50%;
    margin-bottom: 50%;
    width: 3px;
    height: 100px;
    background-color: #ff0000;
    transform-origin: bottom center;
    transition: transform 0.1s ease;
}

.controls {
    display: flex;
    justify-content: space-between;
    color: #fff;
    font-size: 12px;
}
</style>