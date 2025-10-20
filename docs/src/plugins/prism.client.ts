import Prism from 'prismjs';

// テーマの読み込み（正しいパス）
import 'prismjs/themes/prism-okaidia.css';

// プラグインの読み込み（正しいパス）
import 'prismjs/plugins/toolbar/prism-toolbar.css';
import 'prismjs/plugins/toolbar/prism-toolbar';
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard';
import 'prismjs/plugins/inline-color/prism-inline-color.css';
import 'prismjs/plugins/inline-color/prism-inline-color';

// 言語の読み込み
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-powershell';

export default defineNuxtPlugin(_nuxtApp => {
  return {
    provide: {
      Prism
    }
  };
});
