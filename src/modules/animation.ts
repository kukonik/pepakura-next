/**
 * animation.ts
 * 
 * Основной файл экспорта системы анимаций для проекта Pepakura Next.
 */

export { AnimationSystem } from './AnimationSystem';
export { Skeleton } from './Skeleton';
export { Pose } from './Pose';
export { AnimationClip } from './AnimationClip';
export { AnimationTransition } from './AnimationTransition';
export { StateTransitionSystem } from './StateTransitionSystem';

// Типы
export type { AnimationState } from './AnimationSystem';
export type { Bone, BoneTransform } from './Skeleton';
export type { Keyframe } from './AnimationClip';
export type { TransitionOptions } from './AnimationTransition';
export type { AnimationStateConfig, ActiveTransition } from './StateTransitionSystem';