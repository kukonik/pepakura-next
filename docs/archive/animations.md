# Техническая документация: Система анимаций

## Обзор

Система анимаций в Pepakura Next обеспечивает создание, редактирование и воспроизведение скелетных анимаций для 3D моделей. Система поддерживает ключевые кадры, интерполяцию, переходы между анимациями и интеграцию с Three.js.

## Архитектура

### Основные компоненты

#### AnimationSystem
Центральный класс для управления анимациями. Интегрирует все компоненты системы анимаций.

**Расположение:** `src/modules/AnimationSystem.ts`

**Основные методы:**
- `setSkeleton(skeleton: Skeleton)` - Установка скелета для анимации
- `loadClip(clip: AnimationClip)` - Загрузка анимационного клипа
- `play()` - Начало воспроизведения
- `pause()` - Пауза воспроизведения
- `stop()` - Остановка воспроизведения
- `update(delta: number)` - Обновление состояния анимации
- `transitionToClip(targetClip, options?)` - Переход к другому клипу
- `setSpeed(speed: number)` - Установка скорости воспроизведения
- `setLoop(loop: boolean)` - Установка режима зацикливания

#### AnimationClip
Представляет анимационный клип с ключевыми кадрами.

**Расположение:** `src/modules/AnimationClip.ts`

**Основные методы:**
- `addKeyframe(time: number, pose: Pose)` - Добавление ключевого кадра
- `getKeyframes(): Keyframe[]` - Получение всех ключевых кадров
- `getPoseAtTime(time: number): Pose` - Получение позы в определенное время
- `getThreeClip(): THREE.AnimationClip` - Конвертация в Three.js клип
- `fromThreeClip(threeClip): AnimationClip` - Создание из Three.js клипа

#### AnimationTransition
Управляет плавными переходами между анимационными клипами.

**Расположение:** `src/modules/AnimationTransition.ts`

**Основные методы:**
- `startTransition(fromClip, toClip)` - Начало перехода
- `update(delta: number): Pose | null` - Обновление перехода
- `isTransitioningNow(): boolean` - Проверка активного перехода
- `getProgress(): number` - Получение прогресса перехода

#### Skeleton
Управляет иерархией костей скелета.

**Расположение:** `src/modules/Skeleton.ts`

**Основные методы:**
- `addBone(bone: Bone)` - Добавление кости
- `applyPose(pose: Pose)` - Применение позы к скелету
- `getArmature(): THREE.Group` - Получение арматуры Three.js

#### Pose
Представляет состояние скелета в определенный момент времени.

**Расположение:** `src/modules/Pose.ts`

**Основные методы:**
- `setTransform(boneId, transform)` - Установка трансформации кости
- `getTransform(boneId)` - Получение трансформации кости
- `interpolate(poseA, poseB, factor)` - Интерполяция между позами
- `fromThreeSkeleton(skeleton)` - Создание из Three.js скелета

## Создание анимации

### 1. Создание скелета

```typescript
import { Skeleton, Bone } from '@/modules/Skeleton';
import * as THREE from 'three';

const skeleton = new Skeleton();

// Добавление корневой кости
const rootBone: Bone = {
  id: 'root',
  name: 'Root',
  parentId: null,
  position: new THREE.Vector3(0, 0, 0),
  rotation: new THREE.Quaternion(),
  scale: new THREE.Vector3(1, 1, 1)
};
skeleton.addBone(rootBone);

// Добавление дочерней кости
const childBone: Bone = {
  id: 'arm',
  name: 'Arm',
  parentId: 'root',
  position: new THREE.Vector3(0, 1, 0),
  rotation: new THREE.Quaternion(),
  scale: new THREE.Vector3(1, 1, 1)
};
skeleton.addBone(childBone);
```

### 2. Создание анимационного клипа

```typescript
import { AnimationClip } from '@/modules/AnimationClip';
import { Pose } from '@/modules/Pose';

// Создание клипа длительностью 5 секунд
const clip = new AnimationClip('WalkAnimation', 5.0);

// Создание начальной позы
const startPose = new Pose();
startPose.setTransform('arm', {
  position: new THREE.Vector3(0, 1, 0),
  rotation: new THREE.Quaternion(),
  scale: new THREE.Vector3(1, 1, 1)
});

// Создание конечной позы
const endPose = new Pose();
endPose.setTransform('arm', {
  position: new THREE.Vector3(0, 1.5, 0),
  rotation: new THREE.Quaternion().setFromAxisAngle(
    new THREE.Vector3(1, 0, 0),
    Math.PI / 4
  ),
  scale: new THREE.Vector3(1, 1, 1)
});

// Добавление ключевых кадров
clip.addKeyframe(0.0, startPose);
clip.addKeyframe(5.0, endPose);
```

### 3. Инициализация системы анимаций

```typescript
import { AnimationSystem } from '@/modules/AnimationSystem';
import * as THREE from 'three';

const scene = new THREE.Scene();
const animationSystem = new AnimationSystem(scene);

// Установка скелета
animationSystem.setSkeleton(skeleton);

// Загрузка клипа
animationSystem.loadClip(clip);

// Настройка параметров
animationSystem.setSpeed(1.0);
animationSystem.setLoop(true);
```

### 4. Воспроизведение анимации

```typescript
// В игровом цикле
function animate() {
  requestAnimationFrame(animate);
  
  const delta = clock.getDelta(); // Время с последнего кадра
  animationSystem.update(delta);
  
  renderer.render(scene, camera);
}

// Запуск анимации
animationSystem.play();
```

## Переходы между анимациями

### Базовый переход

```typescript
const walkClip = new AnimationClip('Walk', 2.0);
const runClip = new AnimationClip('Run', 1.5);

animationSystem.loadClip(walkClip);
animationSystem.play();

// Переход к бегу через 1 секунду
setTimeout(() => {
  animationSystem.transitionToClip(runClip, {
    duration: 0.3,  // Длительность перехода в секундах
    crossfade: true, // Перекрестное затухание
    easing: AnimationTransition.easeInOutEasing
  });
}, 1000);
```

### Настройка переходов

```typescript
import { AnimationTransition } from '@/modules/AnimationTransition';

// Линейный переход
animationSystem.transitionToClip(targetClip, {
  duration: 0.5,
  easing: AnimationTransition.linearEasing
});

// Ease-in-out переход
animationSystem.transitionToClip(targetClip, {
  duration: 0.5,
  easing: AnimationTransition.easeInOutEasing
});

// Ease-in переход
animationSystem.transitionToClip(targetClip, {
  duration: 0.5,
  easing: AnimationTransition.easeInEasing
});

// Ease-out переход
animationSystem.transitionToClip(targetClip, {
  duration: 0.5,
  easing: AnimationTransition.easeOutEasing
});

// Кастомная функция сглаживания
const customEasing = (t: number): number => {
  return t * t * (3 - 2 * t); // Smoothstep
};

animationSystem.transitionToClip(targetClip, {
  duration: 0.5,
  easing: customEasing
});
```

### Мониторинг переходов

```typescript
// Проверка активного перехода
if (animationSystem.isTransitioningNow()) {
  const progress = animationSystem.getTransitionProgress();
  console.log(`Переход: ${(progress * 100).toFixed(1)}%`);
}
```

## Редактор анимаций

### Компонент AnimationEditor

Редактор анимаций предоставляет визуальный интерфейс для создания и редактирования анимаций.

**Расположение:** `src/components/ui/AnimationEditor.vue`

**Основные функции:**
- Воспроизведение/пауза/остановка анимации
- Добавление и редактирование ключевых кадров
- Управление скоростью воспроизведения
- Настройка зацикливания
- Переходы между клипами
- Визуализация временной шкалы

### Использование редактора

```vue
<template>
  <AnimationEditor 
    :skeleton="skeleton"
    :scene="scene"
  />
</template>

<script setup lang="ts">
import AnimationEditor from '@/components/ui/AnimationEditor.vue';
import { Skeleton } from '@/modules/Skeleton';
import * as THREE from 'three';

const skeleton = new Skeleton();
const scene = new THREE.Scene();
</script>
```

## Интерполяция

### Типы интерполяции

Система автоматически интерполирует между ключевыми кадрами:

1. **Позиция** - Линейная интерполяция (lerp)
2. **Вращение** - Сферическая линейная интерполяция (slerp)
3. **Масштаб** - Линейная интерполяция (lerp)

### Ручная интерполяция

```typescript
import { Pose } from '@/modules/Pose';

const poseA = new Pose();
const poseB = new Pose();

// Интерполяция на 50%
const interpolatedPose = Pose.interpolate(poseA, poseB, 0.5);
```

## Интеграция с Three.js

### Конвертация в Three.js AnimationClip

```typescript
const clip = new AnimationClip('MyAnimation', 5.0);
// ... добавление ключевых кадров

const threeClip = clip.getThreeClip();
const mixer = new THREE.AnimationMixer(skeleton.getArmature());
const action = mixer.clipAction(threeClip);
action.play();
```

### Загрузка из Three.js AnimationClip

```typescript
// Загрузка анимации из GLTF
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';

const loader = new GLTFLoader();
loader.load('/model.gltf', (gltf) => {
  const threeClip = gltf.animations[0];
  const clip = AnimationClip.fromThreeClip(threeClip);
  
  animationSystem.loadClip(clip);
});
```

## Управление состоянием

### Состояние анимации

```typescript
interface AnimationState {
  currentTime: number;    // Текущее время в секундах
  isPlaying: boolean;    // Воспроизводится ли анимация
  isPaused: boolean;     // На паузе ли анимация
  speed: number;         // Скорость воспроизведения
  loop: boolean;          // Зацикливание
}
```

### Получение состояния

```typescript
const state = animationSystem.getState();
console.log('Current time:', state.currentTime);
console.log('Is playing:', state.isPlaying);
console.log('Speed:', state.speed);
```

### Управление временем

```typescript
// Переход к определенному времени
animationSystem.setTime(2.5); // Переход к 2.5 секунде

// Получение текущего времени
const currentTime = animationSystem.getState().currentTime;
```

## Оптимизация производительности

### Кэширование клипов

```typescript
const clipCache = new Map<string, AnimationClip>();

function getClip(name: string): AnimationClip | null {
  return clipCache.get(name) || null;
}

function cacheClip(clip: AnimationClip): void {
  clipCache.set(clip.getName(), clip);
}
```

### Оптимизация обновлений

```typescript
// Обновление только при необходимости
if (animationSystem.getState().isPlaying) {
  animationSystem.update(delta);
}
```

### Ограничение частоты обновлений

```typescript
let lastUpdateTime = 0;
const updateInterval = 1 / 60; // 60 FPS

function update(deltaTime: number) {
  lastUpdateTime += deltaTime;
  
  if (lastUpdateTime >= updateInterval) {
    animationSystem.update(lastUpdateTime);
    lastUpdateTime = 0;
  }
}
```

## Расширение системы

### Добавление новых типов интерполяции

```typescript
// В классе Pose
public static interpolateCustom(
  poseA: Pose, 
  poseB: Pose, 
  factor: number,
  interpolationType: 'linear' | 'ease' | 'bounce'
): Pose {
  // Реализация кастомной интерполяции
}
```

### Добавление событий анимации

```typescript
export class AnimationSystem {
  private events: Map<number, () => void> = new Map();
  
  public addEvent(time: number, callback: () => void): void {
    this.events.set(time, callback);
  }
  
  public update(delta: number): void {
    // ... существующий код
    
    // Проверка событий
    const currentTime = this.currentState.currentTime;
    for (const [eventTime, callback] of this.events) {
      if (Math.abs(currentTime - eventTime) < delta) {
        callback();
      }
    }
  }
}
```

## Отладка

### Визуализация скелета

```typescript
function visualizeSkeleton(skeleton: Skeleton, scene: THREE.Scene): void {
  const armature = skeleton.getArmature();
  
  // Добавление визуализации костей
  armature.traverse((bone) => {
    if (bone instanceof THREE.Bone) {
      const geometry = new THREE.BoxGeometry(0.1, 0.1, 0.1);
      const material = new THREE.MeshBasicMaterial({ color: 0xff0000 });
      const mesh = new THREE.Mesh(geometry, material);
      bone.add(mesh);
    }
  });
  
  scene.add(armature);
}
```

### Логирование состояния

```typescript
function logAnimationState(system: AnimationSystem): void {
  const state = system.getState();
  console.log('Animation State:', {
    time: state.currentTime.toFixed(2),
    playing: state.isPlaying,
    paused: state.isPaused,
    speed: state.speed,
    looping: state.loop,
    transitioning: system.isTransitioningNow()
  });
}
```

## Известные ограничения

1. Поддержка только скелетных анимаций (нет морфинга вершин)
2. Переходы работают только между клипами с одинаковым скелетом
3. Нет поддержки слоев анимаций (blending)
4. Ограниченная поддержка событий анимации

## Связанные компоненты

- `AnimationEditor` - Визуальный редактор анимаций
- `Skeleton` - Управление скелетом
- `Pose` - Представление позы
- `AnimationClip` - Анимационный клип
- `AnimationTransition` - Переходы между анимациями

## Дополнительные ресурсы

- [Three.js Animation Documentation](https://threejs.org/docs/#api/en/animation/AnimationMixer)
- [Skeletal Animation Guide](https://learnopengl.com/Model-Loading/Model)
- [Animation Blending Techniques](https://www.gamedeveloper.com/programming/animation-blending-techniques)

