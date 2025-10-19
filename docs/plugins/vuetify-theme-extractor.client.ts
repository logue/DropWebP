export default defineNuxtPlugin(() => {
  // クライアントサイドでVuetifyテーマスタイルシートを外部化
  if (import.meta.client) {
    const extractThemeStyles = () => {
      const themeStylesheet = document.getElementById('vuetify-theme-stylesheet');
      if (themeStylesheet) {
        // テーマCSSを取得
        const themeCSS = themeStylesheet.textContent || themeStylesheet.innerHTML;

        // 新しい外部スタイルシートを作成
        const link = document.createElement('link');
        link.rel = 'stylesheet';
        link.type = 'text/css';

        // CSSをBlobとして作成し、URLを生成
        const blob = new Blob([themeCSS], { type: 'text/css' });
        const url = URL.createObjectURL(blob);
        link.href = url;

        // headに追加
        document.head.appendChild(link);

        // 元のインラインスタイルを削除
        themeStylesheet.remove();

        console.log('Vuetifyテーマスタイルシートを外部化しました');
      }
    };

    // DOM読み込み完了後に実行
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', extractThemeStyles);
    } else {
      extractThemeStyles();
    }
  }
});
