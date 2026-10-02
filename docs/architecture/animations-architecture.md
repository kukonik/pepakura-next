# Архитектура системы анимаций в Pepakura Next

## Обзор

Система анимаций в Pepakura Next предоставляет мощные возможности для создания и управления анимациями 3D моделей. Система поддерживает скелетную анимацию, морфинг, переходы между состояниями и интеграцию с редактором анимаций.

## Основные компоненты

### 1. AnimationSystem

`AnimationSystem` - это основной класс для управления анимациями. Он отвечает за:

- Управление воспроизведением анимаций (воспроизведение, пауза, остановка)
- Контроль скорости анимаций
- Синхронизацию нескольких анимаций
- Интеграцию с THREE.js AnimationMixer

#### Основные методы:

```typescript
class AnimationSystem {
  // Воспроизведение анимации
  public play(animationName: string, options?: PlayOptions): void
  
  // Пауза анимации
  public pause(animationName: string): void
  
  // Остановка анимации
  public stop(animationName: string): void
  
  // Установка скорости анимации
  public setSpeed(animationName: string, speed: number): void
  
  // Получение текущего состояния анимации
  public getState(animationName: string): AnimationState
  
  // Добавление новой анимации
  public addAnimation(animation: AnimationClip): void
  
  // Удаление анимации
  public removeAnimation(animationName: string): void
}
```

### 2. Skeleton

`Skeleton` - система скелетной анимации. Отвечает за:

- Управление иерархией костей
- Трансформации костей
- Интеграцию с THREE.js Skeleton

#### Основные методы:

```typescript
class Skeleton {
  // Добавление кости
  public addBone(bone: Bone): void
  
  // Удаление кости
  public removeBone(boneName: string): void
  
  // Получение кости по имени
  public getBone(boneName: string): Bone | undefined
  
  // Установка позы скелета
  public setPose(pose: Pose): void
  
  // Получение текущей позы
  public getCurrentPose(): Pose
  
  // Обновление скелета
  public update(): void
}
```

### 3. Pose

`Pose` - представление позы скелета в определенный момент времени.

#### Основные методы:

```typescript
class Pose {
  // Установка трансформации кости
  public setBoneTransform(boneName: string, transform: Transform): void
  
  // Получение трансформации кости
  public getBoneTransform(boneName: string): Transform | undefined
  
  // Интерполяция между двумя позами
  public static interpolate(poseA: Pose, poseB: Pose, factor: number): Pose
  
  // Клонирование позы
  public clone(): Pose
}
```

### 4. AnimationClip

`AnimationClip` - представление анимационного клипа с ключевыми кадрами.

#### Основные методы:

```typescript
class AnimationClip {
  // Добавление ключевого кадра
  public addKeyframe(time: number, pose: Pose): void
  
  // Получение позы в определенное время
  public getPoseAtTime(time: number): Pose
  
  // Получение продолжительности анимации
  public getDuration(): number
  
  // Экспорт в THREE.AnimationClip
  public toThreeJsClip(): THREE.AnimationClip
}
```

### 5. AnimationTransition

`AnimationTransition` - система управления переходами между анимациями.

#### Основные методы:

```typescript
class AnimationTransition {
  // Создание перехода между анимациями
  public static create(fromClip: AnimationClip, toClip: AnimationClip, duration: number, easing?: EasingFunction): AnimationTransition
  
  // Получение позы в определенное время перехода
  public getPoseAtTime(time: number): Pose
  
  // Получение продолжительности перехода
  public getDuration(): number
}
```

### 6. StateTransitionSystem

`StateTransitionSystem` - система управления переходами между состояниями анимаций.

#### Основные методы:

```typescript
class StateTransitionSystem {
  // Добавление состояния
  public addState(stateName: string, clip: AnimationClip): void
  
  // Добавление перехода между состояниями
  public addTransition(fromState: string, toState: string, transition: AnimationTransition): void
  
  // Установка текущего состояния
  public setCurrentState(stateName: string): void
  
  // Получение текущей анимации
  public getCurrentAnimation(): AnimationClip
  
  // Обновление системы
  public update(deltaTime: number): void
}
```

## Интеграция с рендерером

Система анимаций тесно интегрирована с рендерером Three.js и предоставляет следующие возможности:

- Автоматическая настройка скелетов при загрузке моделей
- Поддержка различных типов анимаций
- Оптимизация рендеринга для повышения производительности
- Синхронизация анимаций с физикой и другими системами

## API использования

### Создание скелетной анимации

```typescript
// Создание скелета
const skeleton = new Skeleton();

// Добавление костей
const rootBone = new Bone('root');
const armBone = new Bone('arm');
rootBone.addChild(armBone);
skeleton.addBone(rootBone);

// Создание поз
const pose1 = new Pose();
pose1.setBoneTransform('root', new Transform({ position: new Vector3(0, 0, 0) }));
pose1.setBoneTransform('arm', new Transform({ rotation: new Quaternion() }));

const pose2 = new Pose();
pose2.setBoneTransform('root', new Transform({ position: new Vector3(0, 1, 0) }));
pose2.setBoneTransform('arm', new Transform({ rotation: new Quaternion().setFromAxisAngle(new Vector3(0, 1, 0), Math.PI / 4) }));

// Создание анимационного клипа
const clip = new AnimationClip('wave');
clip.addKeyframe(0, pose1);
clip.addKeyframe(1, pose2);

// Создание системы анимаций
const animationSystem = new AnimationSystem();
animationSystem.addAnimation(clip);

// Воспроизведение анимации
animationSystem.play('wave');
```

### Создание переходов между состояниями

```typescript
// Создание системы переходов между состояниями
const stateSystem = new StateTransitionSystem();

// Добавление состояний
stateSystem.addState('idle', idleClip);
stateSystem.addState('walk', walkClip);

// Создание перехода
const transition = AnimationTransition.create(idleClip, walkClip, 0.5);
stateSystem.addTransition('idle', 'walk', transition);

// Установка текущего состояния
stateSystem.setCurrentState('idle');

// Переключение состояния
stateSystem.setCurrentState('walk');
```

## Редактор анимаций

Система включает визуальный редактор анимаций с возможностями:

- Таймлайн для управления ключевыми кадрами
- Редактор свойств костей
- Предпросмотр анимаций в реальном времени
- Инструменты для создания и редактирования переходов
- Поддержка нескольких дорожек анимации

## Производительность

Система анимаций оптимизирована для:

- Эффективной работы с большими скелетами
- Минимизации пересчетов трансформаций
- Кэширования анимационных данных
- Поддержки LOD (уровней детализации) для анимаций

## Расширяемость

Система спроектирована для легкого расширения:

- Добавление новых типов анимаций
- Создание пользовательских переходов
- Интеграция с внешними библиотеками анимаций
- Поддержка новых форматов анимационных данных

## Заключение

Система анимаций Pepakura Next предоставляет мощные и гибкие возможности для создания и управления анимациями 3D моделей. Она сочетает в себе простоту использования с возможностью тонкой настройки, что делает её подходящей как для начинающих пользователей, так и для профессионалов.