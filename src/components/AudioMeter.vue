<template>
  <div class="audio-meter">
    <div class="meter vu-meter">
      <div class="meter-scale">
        <div v-for="value in scaleValues" :key="value" class="scale-mark">
            {{ value }}
        </div>
      </div>
      <div class="needle" :style="{ transform: `rotate(${vuRotation}deg)` }"></div>
    </div>
    <div class="meter ppm-meter">
      <div class="bar" :style="{ width: `${ppmWidth}%` }"></div>
    </div>
    <div class="controls">
      <div>TRIM LEFT</div>
      <div>TRIM RIGHT</div>
    </div>
  </div>
</template>

<script setup>
  import { computed } from 'vue';
  
  const props = defineProps({
    averageLevel: {
      type: Number,
      required: true,
      validator: (value) => value >= -20 && value <= 3
    },
    peakLevel: {
      type: Number,
      required: true,
      validator: (value) => value >= -20 && value <= 3
    }
  });
  
  const scaleValues = [-100, -75, -50, -10, -5, 0, 5, 10, 50, 75, 100];
  
  const vuRotation = computed(() => {
    // Map the average level to rotation degrees
    const minDegree = -45;
    const maxDegree = 45;
    const minLevel = -20;
    const maxLevel = 3;
    return minDegree + (props.averageLevel - minLevel) / (maxLevel - minLevel) * (maxDegree - minDegree);
  });
  
  const ppmWidth = computed(() => {
    // Map the peak level to percentage width
    const minLevel = -20;
    const maxLevel = 3;
    return Math.max(0, Math.min(100, (props.peakLevel - minLevel) / (maxLevel - minLevel) * 100));
  });
</script>

<style scoped>
  .audio-meter {
    width: 300px;
    background-color: #1e1e1e;
    padding: 10px;
    border-radius: 5px;
  }
  
  .meter {
    position: relative;
    height: 100px;
    background-color: #000;
    margin-bottom: 10px;
    overflow: hidden;
  }
  
  .vu-meter {
    border-radius: 50% 50% 0 0;
  }
  
  .meter-scale {
    position: absolute;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-between;
    padding: 0 10px;
    color: #fff;
    font-size: 12px;
  }
  
  .needle {
    position: absolute;
    bottom: 0;
    left: 50%;
    width: 2px;
    height: 80px;
    background-color: #ff0000;
    transform-origin: bottom center;
    transition: transform 0.1s ease;
  }
  
  .ppm-meter .bar {
    height: 100%;
    background-color: #00ff00;
    transition: width 0.1s ease;
  }
  
  .controls {
    display: flex;
    justify-content: space-between;
    color: #fff;
    font-size: 12px;
  }
</style>