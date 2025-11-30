/**
 * Progress event payload from encoding process
 */
export interface ProgressEvent {
  /** Progress percentage (0-100) */
  percent: number;
  /** Current encoding stage description */
  stage: string;
  /** Progress status */
  status: 'progress' | 'complete' | 'error';
}

/**
 * Encoder progress support information
 */
export const EncoderProgressSupport = {
  /** Formats that support progress monitoring */
  supported: ['webp', 'png'] as const,

  /** Formats that do not support progress monitoring */
  unsupported: ['avif', 'jpeg', 'jxl'] as const,

  /**
   * Check if a format supports progress monitoring
   * @param format - Format name
   * @returns True if progress monitoring is supported
   */
  isSupported(format: string): boolean {
    return this.supported.includes(format as any);
  }
} as const;

/**
 * Example usage in Vue component:
 *
 * ```typescript
 * import { listen } from '@tauri-apps/api/event';
 * import type { ProgressEvent } from './types/ProgressEvent';
 *
 * // Listen for progress events
 * const unlisten = await listen<ProgressEvent>('encoding-progress', (event) => {
 *   const { percent, stage, status } = event.payload;
 *
 *   if (status === 'progress') {
 *     console.log(`Progress: ${percent.toFixed(1)}% - ${stage}`);
 *     // Update UI progress bar
 *   } else if (status === 'complete') {
 *     console.log('Encoding completed!');
 *   } else if (status === 'error') {
 *     console.error(`Encoding failed: ${stage}`);
 *   }
 * });
 *
 * // Call the command
 * try {
 *   const result = await invoke<Uint8Array>('convert_with_progress', {
 *     data: imageData,
 *     options: encodeOptions
 *   });
 *   console.log('Conversion successful:', result);
 * } catch (error) {
 *   console.error('Conversion failed:', error);
 * } finally {
 *   // Clean up listener
 *   unlisten();
 * }
 * ```
 */
