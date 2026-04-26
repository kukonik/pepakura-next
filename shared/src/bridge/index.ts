import type { PlatformBridge } from './PlatformBridge';

export * from './types';
export * from './PlatformBridge';
export * from './TauriBridge';
export * from './WasmBridge';

let _instance: PlatformBridge | null = null;

/**
 * Set the global bridge instance.
 * Should be called during app initialization.
 */
export const setBridge = (b: PlatformBridge) => {
  _instance = b;
};

/**
 * Get the global bridge instance.
 * Throws if bridge hasn't been initialized.
 */
export const getBridge = (): PlatformBridge => {
  if (!_instance) {
    throw new Error('Bridge not initialized. Call setBridge() first.');
  }
  return _instance;
};

/**
 * Utility to check if bridge is initialized.
 */
export const hasBridge = (): boolean => _instance !== null;