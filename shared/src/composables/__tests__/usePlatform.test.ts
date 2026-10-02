import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { usePlatform } from '../usePlatform';
import type { PlatformBridge } from '../../bridge';

// Mock the bridge module
vi.mock('../../bridge/TauriBridge', () => ({
  createTauriBridge: vi.fn(() => ({
    invoke: vi.fn(),
    invokeWithResult: vi.fn(),
  })),
}));

vi.mock('../../bridge/WasmBridge', () => ({
  createWasmBridge: vi.fn(() => ({
    invoke: vi.fn(),
    invokeWithResult: vi.fn(),
  })),
}));

vi.mock('../../bridge', () => ({
  setBridge: vi.fn(),
}));

describe('usePlatform', () => {
  beforeEach(() => {
    // Clear mocks
    vi.clearAllMocks();
    // Reset global window.__TAURI__ for each test
    (window as any).__TAURI__ = undefined;
    (window as any).__TAURI_INVOKE__ = undefined;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('returns invoke function', () => {
    const { invoke } = usePlatform();
    expect(invoke).toBeDefined();
    expect(typeof invoke).toBe('function');
  });

  it('returns invokeWithResult function', () => {
    const { invokeWithResult } = usePlatform();
    expect(invokeWithResult).toBeDefined();
    expect(typeof invokeWithResult).toBe('function');
  });

  it('detects desktop environment when __TAURI__ is present', () => {
    (window as any).__TAURI__ = { invoke: vi.fn() };
    const { isDesktop } = usePlatform();
    expect(isDesktop.value).toBe(true);
  });

  it('detects web environment when __TAURI__ is absent', () => {
    (window as any).__TAURI__ = undefined;
    const { isWeb } = usePlatform();
    expect(isWeb.value).toBe(true);
  });

  it('initializes bridge on init()', async () => {
    const { init, isInitialized } = usePlatform();
    expect(isInitialized.value).toBe(false);
    await init();
    expect(isInitialized.value).toBe(true);
  });

  it('handles bridge errors', async () => {
    // Mock a failing bridge
    const { invokeWithResult } = usePlatform();
    // Since bridge is not initialized, it will try to init and create a bridge
    // The mock bridge's invokeWithResult will return a success: false result
    // We need to set up the mock accordingly
    const mockBridge: PlatformBridge = {
      invoke: vi.fn(),
      invokeWithResult: vi.fn().mockResolvedValue({ success: false, error: 'Command not found' }),
    };
    vi.mocked(require('../../bridge/TauriBridge').createTauriBridge).mockReturnValue(mockBridge);
    // Set desktop environment
    (window as any).__TAURI__ = {};
    const result = await invokeWithResult('invalid_command', {});
    expect(result.success).toBe(false);
    expect(result.error).toBeDefined();
  });

  it('invoke calls bridge.invoke', async () => {
    const mockInvoke = vi.fn().mockResolvedValue('test result');
    const mockBridge: PlatformBridge = {
      invoke: mockInvoke,
      invokeWithResult: vi.fn(),
    };
    vi.mocked(require('../../bridge/TauriBridge').createTauriBridge).mockReturnValue(mockBridge);
    (window as any).__TAURI__ = {};
    const { invoke } = usePlatform();
    const result = await invoke('test_command', { arg: 1 });
    expect(mockInvoke).toHaveBeenCalledWith('test_command', { arg: 1 });
    expect(result).toBe('test result');
  });
});