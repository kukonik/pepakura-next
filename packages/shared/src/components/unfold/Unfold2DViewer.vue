<template>
  <div ref="canvasRef" class="unfold-2d-container">
    <canvas ref="canvasEl" class="paper-canvas"></canvas>
    <div class="unfold-info">
      <span>Швов выбрано: {{ cutsCount }}</span>
      <span style="font-size: 0.8em; color: #888;">(Двойной клик в 3D для выбора)</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { useUnfoldState } from "../../composables/useUnfoldState";

const canvasEl = ref<HTMLCanvasElement>();
const canvasRef = ref<HTMLDivElement>();
const unfoldState = useUnfoldState();

let ctx: CanvasRenderingContext2D | null = null;

const cutsCount = ref(0);

onMounted(() => {
  if (canvasEl.value) {
    ctx = canvasEl.value.getContext("2d");
    resizeCanvas();
    drawPaper();
  }
  window.addEventListener("resize", resizeCanvas);
});

onUnmounted(() => {
  window.removeEventListener("resize", resizeCanvas);
});

watch(() => unfoldState.getCutsCount(), (newCount) => {
  cutsCount.value = newCount;
  // В будущем здесь будет триггер пересчета геометрии развертки
});

function resizeCanvas() {
  if (canvasRef.value && canvasEl.value && ctx) {
    canvasEl.value.width = canvasRef.value.clientWidth;
    canvasEl.value.height = canvasRef.value.clientHeight;
    drawPaper();
  }
}

function drawPaper() {
  if (!ctx) return;
  const w = canvasEl.value!.width;
  const h = canvasEl.value!.height;

  // Очистка
  ctx.clearRect(0, 0, w, h);

  // Рисуем "сетку бумаги"
  ctx.strokeStyle = "#333";
  ctx.lineWidth = 1;
  const gridSize = 20;
  
  ctx.beginPath();
  for (let x = 0; x <= w; x += gridSize) {
    ctx.moveTo(x, 0);
    ctx.lineTo(x, h);
  }
  for (let y = 0; y <= h; y += gridSize) {
    ctx.moveTo(0, y);
    ctx.lineTo(w, y);
  }
  ctx.stroke();

  // Заглушка для будущего 2D вывода
  if (cutsCount.value > 0) {
    ctx.fillStyle = "#007acc";
    ctx.font = "16px monospace";
    ctx.fillText("Ожидание алгоритма развертки...", 20, 40);
  } else {
    ctx.fillStyle = "#555";
    ctx.font = "16px monospace";
    ctx.fillText("Перейдите во вкладку 3D и выберите швы", 20, 40);
  }
}
</script>

<style scoped>
.unfold-2d-container {
  position: relative;
  width: 100%;
  height: 100%;
  background-color: #252526;
  overflow: hidden;
}
.paper-canvas {
  display: block;
  width: 100%;
  height: 100%;
}
.unfold-info {
  position: absolute;
  top: 10px;
  left: 10px;
  background: rgba(0, 0, 0, 0.6);
  padding: 5px 10px;
  border-radius: 4px;
  color: #fff;
  font-family: monospace;
  pointer-events: none;
}
</style>
