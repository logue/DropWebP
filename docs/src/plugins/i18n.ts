import { createI18n } from 'vue-i18n';

// 言語メッセージを直接定義
const messages = {
  ja: {
    common: {
      home: 'ホーム',
      test: 'テスト',
      settings: '設定',
      about: 'について',
      language: '言語',
      copyright: '2025 © Logue'
    },
    header: {
      title: 'Drop Compress Image Help',
      home: 'ホーム',
      test: 'テスト'
    },
    footer: {
      copyright: '2025 © Logue'
    },
    home: {
      title: 'Drop Compress Image ヘルプサイト',
      subtitle: '画像圧縮アプリのヘルプとガイド',
      start_button: '開始する',
      features: {
        multiple_formats: {
          title: '複数形式対応',
          description: 'WebP、AVIF、JXLなど最新の画像形式に対応'
        },
        high_speed: {
          title: '高速変換',
          description: 'Rust基盤で高速な画像処理を実現'
        },
        drag_drop: {
          title: 'ドラッグ&ドロップ',
          description: '簡単な操作で画像を一括変換'
        }
      }
    },
    test: {
      title: 'Vuetify テストページ',
      description: 'vuetify-nuxt-moduleが正常に動作するかテストするページです。',
      button_test: 'ボタンテスト',
      icon_test: 'アイコンテスト',
      card_test: 'カードテスト',
      form_test: 'フォームテスト',
      card_title: 'カード',
      card_content: 'これはテストカードです。Vuetifyのマテリアルデザインが適用されています。',
      action_button: 'アクション',
      test_input: 'テスト入力',
      test_select: 'テスト選択',
      success_message: 'vuetify-nuxt-moduleが正常に動作しています！',
      select_options: {
        option1: 'オプション1',
        option2: 'オプション2',
        option3: 'オプション3'
      },
      form_name: '名前',
      form_email: 'メールアドレス',
      form_message: 'メッセージ',
      form_submit: '送信'
    },
    messages: {
      success: '成功',
      error: 'エラー',
      warning: '警告',
      info: '情報'
    },
    getting_started: {
      title: 'はじめに',
      subtitle: 'Drop Compress Image の使い方を学びましょう',
      meta_description: 'Drop Compress Image の使い方とインストール方法',
      download_button: 'ダウンロード',
      installation: {
        title: 'インストール方法',
        step1_title: 'アプリをダウンロード',
        step1_desc: 'GitHub Releases から最新版をダウンロードしてください。',
        step2_title: 'インストール実行',
        step2_desc: 'ダウンロードしたファイルを実行してインストールを開始します。',
        step3_title: '完了',
        step3_desc: 'インストールが完了したらアプリを起動できます。'
      },
      usage: {
        title: '使用方法',
        methods: {
          drag_drop: {
            title: 'ドラッグ&ドロップ',
            description: 'ファイルをアプリにドラッグ&ドロップするだけで変換開始'
          },
          file_menu: {
            title: 'ファイルメニュー',
            description: 'メニューからファイルを選択して変換'
          },
          batch_convert: {
            title: '一括変換',
            description: '複数のファイルを同時に変換可能'
          },
          settings: {
            title: '設定カスタマイズ',
            description: '品質や形式などの設定を細かく調整'
          }
        }
      },
      features: {
        title: 'サポート機能',
        list: {
          webp_support: {
            title: 'WebP 対応',
            description: 'Google開発の次世代画像形式WebPに完全対応'
          },
          avif_support: {
            title: 'AVIF 対応',
            description: 'AV1ベースの高効率画像形式AVIFをサポート'
          },
          jxl_support: {
            title: 'JPEG XL 対応',
            description: '最新の画像形式JPEG XLに対応'
          },
          batch_processing: {
            title: 'バッチ処理',
            description: '複数ファイルの同時処理が可能'
          },
          quality_control: {
            title: '品質制御',
            description: 'ファイルサイズと画質のバランスを調整'
          },
          cross_platform: {
            title: 'クロスプラットフォーム',
            description: 'Windows、macOS、Linuxで動作'
          }
        }
      }
    }
  },
  en: {
    common: {
      home: 'Home',
      test: 'Test',
      settings: 'Settings',
      about: 'About',
      language: 'Language',
      copyright: '2025 © Logue'
    },
    header: {
      title: 'Drop Compress Image Help',
      home: 'Home',
      test: 'Test'
    },
    footer: {
      copyright: '2025 © Logue'
    },
    home: {
      title: 'Drop Compress Image Help Site',
      subtitle: 'Help and guide for image compression app',
      start_button: 'Get Started',
      features: {
        multiple_formats: {
          title: 'Multiple Format Support',
          description: 'Support for latest image formats like WebP, AVIF, JXL'
        },
        high_speed: {
          title: 'High-Speed Conversion',
          description: 'Fast image processing powered by Rust'
        },
        drag_drop: {
          title: 'Drag & Drop',
          description: 'Batch convert images with simple operations'
        }
      }
    },
    test: {
      title: 'Vuetify Test Page',
      description: 'This page tests if vuetify-nuxt-module is working properly.',
      button_test: 'Button Test',
      icon_test: 'Icon Test',
      card_test: 'Card Test',
      form_test: 'Form Test',
      card_title: 'Card',
      card_content: "This is a test card. Vuetify's Material Design is applied.",
      action_button: 'Action',
      test_input: 'Test Input',
      test_select: 'Test Select',
      success_message: 'vuetify-nuxt-module is working properly!',
      select_options: {
        option1: 'Option 1',
        option2: 'Option 2',
        option3: 'Option 3'
      },
      form_name: 'Name',
      form_email: 'Email',
      form_message: 'Message',
      form_submit: 'Submit'
    },
    messages: {
      success: 'Success',
      error: 'Error',
      warning: 'Warning',
      info: 'Information'
    }
  },
  ko: {
    common: {
      home: '홈',
      test: '테스트',
      settings: '설정',
      about: '정보',
      language: '언어',
      copyright: '2025 © Logue'
    },
    header: {
      title: 'Drop Compress Image 도움말',
      home: '홈',
      test: '테스트'
    },
    footer: {
      copyright: '2025 © Logue'
    },
    home: {
      title: 'Drop Compress Image 도움말 사이트',
      subtitle: '이미지 압축 앱의 도움말 및 가이드',
      start_button: '시작하기',
      features: {
        multiple_formats: {
          title: '다중 형식 지원',
          description: 'WebP, AVIF, JXL 등 최신 이미지 형식을 지원'
        },
        high_speed: {
          title: '고속 변환',
          description: 'Rust 기반의 빠른 이미지 처리 구현'
        },
        drag_drop: {
          title: '드래그 & 드롭',
          description: '간단한 조작으로 이미지 일괄 변환'
        }
      }
    },
    test: {
      title: 'Vuetify 테스트 페이지',
      description: 'vuetify-nuxt-module이 정상적으로 작동하는지 테스트하는 페이지입니다.',
      button_test: '버튼 테스트',
      icon_test: '아이콘 테스트',
      card_test: '카드 테스트',
      form_test: '폼 테스트',
      card_title: '카드',
      card_content: '이것은 테스트 카드입니다. Vuetify의 Material Design이 적용되어 있습니다.',
      action_button: '액션',
      test_input: '테스트 입력',
      test_select: '테스트 선택',
      success_message: 'vuetify-nuxt-module이 정상적으로 작동하고 있습니다!',
      select_options: {
        option1: '옵션 1',
        option2: '옵션 2',
        option3: '옵션 3'
      },
      form_name: '이름',
      form_email: '이메일',
      form_message: '메시지',
      form_submit: '제출'
    },
    messages: {
      success: '성공',
      error: '오류',
      warning: '경고',
      info: '정보'
    }
  },
  'zh-tw': {
    common: {
      home: '首頁',
      test: '測試',
      settings: '設定',
      about: '關於',
      language: '語言',
      copyright: '2025 © Logue'
    },
    header: {
      title: 'Drop Compress Image 說明',
      home: '首頁',
      test: '測試'
    },
    footer: {
      copyright: '2025 © Logue'
    },
    home: {
      title: 'Drop Compress Image 說明網站',
      subtitle: '圖像壓縮應用程式的說明與指南',
      start_button: '開始使用',
      features: {
        multiple_formats: {
          title: '多格式支援',
          description: '支援 WebP、AVIF、JXL 等最新圖像格式'
        },
        high_speed: {
          title: '高速轉換',
          description: '基於 Rust 的快速圖像處理實現'
        },
        drag_drop: {
          title: '拖放操作',
          description: '簡單操作即可批次轉換圖像'
        }
      }
    },
    test: {
      title: 'Vuetify 測試頁面',
      description: '這個頁面用來測試 vuetify-nuxt-module 是否正常運作。',
      button_test: '按鈕測試',
      icon_test: '圖示測試',
      card_test: '卡片測試',
      form_test: '表單測試',
      card_title: '卡片',
      card_content: '這是一個測試卡片。已套用 Vuetify 的 Material Design。',
      action_button: '動作',
      test_input: '測試輸入',
      test_select: '測試選擇',
      success_message: 'vuetify-nuxt-module 正常運作中！',
      select_options: {
        option1: '選項 1',
        option2: '選項 2',
        option3: '選項 3'
      },
      form_name: '姓名',
      form_email: '電子郵件',
      form_message: '訊息',
      form_submit: '提交'
    },
    messages: {
      success: '成功',
      error: '錯誤',
      warning: '警告',
      info: '資訊'
    }
  }
};

export default defineNuxtPlugin(nuxtApp => {
  // URLから言語コードを取得する関数
  const getLocaleFromRoute = () => {
    if (!process.client) return 'ja';

    const path = window.location.pathname;
    const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];

    // パスの先頭から言語コードを抽出（例: /en/getting-started -> en）
    const pathSegments = path.split('/').filter(Boolean);
    const firstSegment = pathSegments[0];

    if (supportedLocales.includes(firstSegment!)) {
      return firstSegment;
    }

    // パスに言語コードがない場合はデフォルト（日本語）
    return 'ja';
  };

  // ブラウザの言語設定を取得（フォールバック用）
  const browserLocale = process.client ? navigator.language.slice(0, 2) || 'ja' : 'ja';

  // URLから言語を優先的に取得、なければブラウザ設定
  const routeLocale = getLocaleFromRoute();
  let defaultLocale = routeLocale;

  // 言語コード正規化
  if (browserLocale === 'zh' && defaultLocale === 'ja') {
    defaultLocale = 'zh-tw';
  }

  // 対応言語でない場合は日本語をデフォルトに
  const supportedLocales = ['ja', 'en', 'ko', 'zh-tw'];
  if (!supportedLocales.includes(defaultLocale!)) {
    defaultLocale = 'ja';
  }

  // i18nインスタンスを作成
  const i18n = createI18n({
    legacy: false,
    locale: defaultLocale,
    fallbackLocale: 'ja',
    messages,
    globalInjection: true
  });

  // VueアプリインスタンスにVue I18nをインストール
  nuxtApp.vueApp.use(i18n);

  // i18nインスタンスをNuxtアプリで利用可能にする
  return {
    provide: {
      i18n: i18n.global
    }
  };
});
