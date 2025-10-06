import {
  sendNotification,
  isPermissionGranted,
  requestPermission
} from '@tauri-apps/plugin-notification';

/**
 * デスクトップ通知を送信するためのcomposable
 */
export const useNotification = () => {
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
        await sendNotification({
          title,
          body,
          icon
        });
      } else {
        console.warn('通知権限が許可されていません');
      }
    } catch (error) {
      console.error('通知の送信に失敗しました:', error);
    }
  };

  /**
   * 画像変換完了通知
   */
  const notifyConversionComplete = async (fileName: string, format: string) => {
    const title = '画像変換完了';
    const body = `${fileName} を ${format.toUpperCase()} 形式に変換しました`;

    await notify(title, body);
  };

  /**
   * 一括変換完了通知
   */
  const notifyBatchComplete = async (count: number, format: string) => {
    const title = 'バッチ変換完了';
    const body = `${count} 個のファイルを ${format.toUpperCase()} 形式に変換しました`;

    await notify(title, body);
  };

  /**
   * エラー通知
   */
  const notifyError = async (message: string) => {
    const title = 'エラー';

    await notify(title, message);
  };

  return {
    notify,
    notifyConversionComplete,
    notifyBatchComplete,
    notifyError
  };
};
