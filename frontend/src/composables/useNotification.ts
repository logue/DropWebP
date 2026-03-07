import type { ComposerTranslation } from 'vue-i18n';

import {
  sendNotification,
  isPermissionGranted,
  requestPermission
} from '@tauri-apps/plugin-notification';

/**
 * デスクトップ通知を送信するためのcomposable
 */
export const useNotification = (t: ComposerTranslation) => {
  /**
   * 通知権限を要求し、通知を送信
   */
  const notify = async (title: string, body?: string, icon?: string) => {
    try {
      // 通知権限を確認
      let permissionGranted = await isPermissionGranted();

      // 権限がない場合は要求
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
      }

      if (permissionGranted) {
        // 通知を送信
        await sendNotification({ title, body, icon });
      }
    } catch (error) {
      console.error('通知の送信に失敗しました:', error);
    }
  };

  /**
   * 画像変換完了通知
   */
  const notifyConversionComplete = async (fileName: string, format: string) => {
    // 画像変換処理が完了したことを通知
    await notify(
      t('notification.complete.title'),
      t('notification.complete.message', { file: fileName, format: format.toUpperCase() })
    );
  };

  /**
   * 一括変換完了通知
   */
  const notifyBatchComplete = async (count: number, format: string) => {
    // 一括変換処理が完了したことを通知
    await notify(
      t('notification.batch_complete.title'),
      t('notification.batch_complete.message', { count, format: format.toUpperCase() })
    );
  };

  /**
   * エラー通知
   */
  const notifyError = async (message: string) => {
    await notify(t('notification.error.title'), message);
  };

  return {
    notify,
    notifyConversionComplete,
    notifyBatchComplete,
    notifyError
  };
};
