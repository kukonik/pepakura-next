/**
 * Хранилище швов
 */

import { defineStore } from 'pinia';
import type { SeamSet, AiBackendConfig } from '@pepakura-next/shared';

/**
 * Хранилище швов
 */
export const useSeamsStore = defineStore('seams', {
  state: () => ({
    // Набор швов
    seamSet: { edges: [] } as SeamSet,
    
    // Флаг режима редактирования швов
    isEditing: false,
    
    // Конфигурация AI бэкенда
    aiConfig: {
      backendType: 'ollama',
      endpoint: 'http://localhost:11434/api/chat',
      model: 'llama3',
      generationParams: {
        temperature: 0.7,
        topP: 0.9,
        maxTokens: 1000,
      },
    } as AiBackendConfig,
  }),
  
  actions: {
    /**
     * Установка набора швов
     * @param seamSet Новый набор швов
     */
    setSeamSet(seamSet: SeamSet) {
      this.seamSet = seamSet;
    },
    
    /**
     * Добавление шва
     * @param edge Ребро для добавления в швы
     */
    addSeam(edge: any) {
      // Проверяем, что ребро еще не добавлено
      const isAlreadyAdded = this.seamSet.edges.some(e => 
        e.vertex1 === edge.vertex1 && e.vertex2 === edge.vertex2
      );
      
      if (!isAlreadyAdded) {
        // Добавляем ребро в набор швов
        this.seamSet.edges.push(edge);
      }
    },
    
    /**
     * Удаление шва
     * @param edge Ребро для удаления из швов
     */
    removeSeam(edge: any) {
      // Удаляем ребро из набора швов
      this.seamSet.edges = this.seamSet.edges.filter(e => 
        !(e.vertex1 === edge.vertex1 && e.vertex2 === edge.vertex2)
      );
    },
    
    /**
     * Начало режима редактирования швов
     */
    startEditing() {
      this.isEditing = true;
    },
    
    /**
     * Завершение режима редактирования швов
     */
    stopEditing() {
      this.isEditing = false;
    },
    
    /**
     * Установка конфигурации AI бэкенда
     * @param config Новая конфигурация AI бэкенда
     */
    setAiConfig(config: AiBackendConfig) {
      this.aiConfig = config;
    },
  },
});