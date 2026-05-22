/**
 * StateTransitionSystem.ts
 * 
 * Система управления переходами между состояниями анимаций в проекте Pepakura Next.
 * Обеспечивает плавные переходы между различными анимационными состояниями.
 */

import * as THREE from "three";
import { AnimationSystem } from "./AnimationSystem";
import { AnimationClip } from "./AnimationClip";
import { AnimationTransition, TransitionOptions } from "./AnimationTransition";
import { Pose } from "./Pose";

export interface AnimationStateConfig {
  name: string;
  clip: AnimationClip;
  transitions: {
    [targetState: string]: TransitionOptions;
  };
}

export interface ActiveTransition {
  fromState: string;
  toState: string;
  transition: AnimationTransition;
}

export class StateTransitionSystem {
  private animationSystem: AnimationSystem;
  private states: Map<string, AnimationStateConfig>;
  private currentState: string | null = null;
  private activeTransition: ActiveTransition | null = null;
  private defaultTransitionOptions: TransitionOptions;

  constructor(animationSystem: AnimationSystem) {
    this.animationSystem = animationSystem;
    this.states = new Map();
    this.defaultTransitionOptions = {
      duration: 0.3,
      easing: AnimationTransition.easeInOutEasing,
      crossfade: true
    };
  }

  /**
   * Добавляет новое анимационное состояние
   */
  public addState(stateConfig: AnimationStateConfig): void {
    this.states.set(stateConfig.name, stateConfig);
  }

  /**
   * Удаляет анимационное состояние
   */
  public removeState(stateName: string): void {
    this.states.delete(stateName);
    
    // Если удаляемое состояние является текущим, останавливаем анимацию
    if (this.currentState === stateName) {
      this.currentState = null;
      this.animationSystem.stop();
    }
    
    // Если удаляемое состояние является частью активного перехода, отменяем переход
    if (this.activeTransition && 
        (this.activeTransition.fromState === stateName || 
         this.activeTransition.toState === stateName)) {
      this.activeTransition = null;
    }
  }

  /**
   * Получает конфигурацию состояния
   */
  public getStateConfig(stateName: string): AnimationStateConfig | undefined {
    return this.states.get(stateName);
  }

  /**
   * Получает все доступные состояния
   */
  public getAvailableStates(): string[] {
    return Array.from(this.states.keys());
  }

  /**
   * Переходит к указанному состоянию с плавным переходом
   */
  public async transitionToState(targetState: string): Promise<void> {
    // Проверяем, существует ли целевое состояние
    const targetStateConfig = this.states.get(targetState);
    if (!targetStateConfig) {
      throw new Error(`State '${targetState}' not found`);
    }

    // Если уже в этом состоянии, ничего не делаем
    if (this.currentState === targetState && !this.activeTransition) {
      return;
    }

    // Определяем параметры перехода
    let transitionOptions = this.defaultTransitionOptions;
    
    // Если есть текущее состояние, проверяем специфичные параметры перехода
    if (this.currentState) {
      const currentStateConfig = this.states.get(this.currentState);
      if (currentStateConfig && currentStateConfig.transitions[targetState]) {
        transitionOptions = currentStateConfig.transitions[targetState];
      }
    }

    // Создаем объект перехода
    const transition = new AnimationTransition(transitionOptions);
    
    // Получаем текущий клип (если есть)
    const fromClip = this.currentState ? 
      this.states.get(this.currentState)?.clip : null;
    
    // Начинаем переход
    transition.startTransition(fromClip, targetStateConfig.clip);
    
    // Сохраняем информацию о переходе
    this.activeTransition = {
      fromState: this.currentState || '',
      toState: targetState,
      transition: transition
    };

    // Загружаем целевой клип в анимационную систему
    this.animationSystem.loadClip(targetStateConfig.clip);
    
    // Обновляем текущее состояние
    this.currentState = targetState;
  }

  /**
   * Обновляет систему переходов
   * @param delta Время в секундах с последнего кадра
   */
  public update(delta: number): void {
    // Если есть активный переход, обновляем его
    if (this.activeTransition) {
      const pose = this.activeTransition.transition.update(delta);
      
      // Если переход завершен, очищаем его
      if (!this.activeTransition.transition.isTransitioningNow()) {
        this.activeTransition = null;
      }
      
      // Если получили позу, применяем её
      if (pose) {
        this.animationSystem.setPose(pose);
      }
    }
    
    // Обновляем анимационную систему
    this.animationSystem.update(delta);
  }

  /**
   * Получает текущее состояние
   */
  public getCurrentState(): string | null {
    return this.currentState;
  }

  /**
   * Проверяет, выполняется ли переход
   */
  public isTransitioning(): boolean {
    return this.activeTransition !== null;
  }

  /**
   * Получает прогресс текущего перехода (0.0 - 1.0)
   */
  public getTransitionProgress(): number {
    if (this.activeTransition) {
      return this.activeTransition.transition.getProgress();
    }
    return 1.0;
  }

  /**
   * Устанавливает параметры перехода по умолчанию
   */
  public setDefaultTransitionOptions(options: TransitionOptions): void {
    this.defaultTransitionOptions = { ...options };
  }

  /**
   * Создает циклический переход между состояниями
   */
  public createCycleTransition(states: string[], options?: TransitionOptions): void {
    for (let i = 0; i < states.length; i++) {
      const fromState = states[i];
      const toState = states[(i + 1) % states.length];
      
      const fromStateConfig = this.states.get(fromState);
      if (fromStateConfig) {
        fromStateConfig.transitions[toState] = options || this.defaultTransitionOptions;
      }
    }
  }

  /**
   * Создает двусторонний переход между двумя состояниями
   */
  public createBidirectionalTransition(
    stateA: string, 
    stateB: string, 
    options?: TransitionOptions
  ): void {
    const stateAConfig = this.states.get(stateA);
    const stateBConfig = this.states.get(stateB);
    
    if (stateAConfig) {
      stateAConfig.transitions[stateB] = options || this.defaultTransitionOptions;
    }
    
    if (stateBConfig) {
      stateBConfig.transitions[stateA] = options || this.defaultTransitionOptions;
    }
  }
}