# アクセシビリティインジケータ実装 (Accessibility Indicators Implementation)

## 概要 (Overview)

視力の弱い（ロービジョン）ユーザーにやさしいアプリケーションを目指し、視覚的にわかりやすい一貫したインジケータシステムを実装しました。

Implemented a consistent visual indicator system to make the application more accessible for users with low vision.

## 設計理念 (Design Philosophy)

### ユニバーサルデザインの原則
1. **視認性の向上** - 大きく、はっきりとしたインジケータ
2. **一貫性** - すべての入力・選択項目で同じ方式を採用
3. **直感性** - 視覚的にわかりやすい表現（塗りつぶしの丸）
4. **色覚バリアフリー** - 黒い縁取りで色に依存せず形状のコントラストで認識可能

### ターゲットユーザー
- ロービジョンの方
- 高齢者
- 認知機能の低下がある方
- 色覚異常（色弱・色盲）の方
- すべてのユーザー（ユニバーサルデザイン）

## 実装内容 (Implementation Details)

### 1. 言語メニューの選択インジケータ

#### 視覚的表現
- **非選択状態**: 空の丸（○）- 薄いグレーの枠線のみ
- **選択状態**: 塗りつぶしの丸（●）- 緑色の塗りつぶし + 黒い縁取り

#### デザイン理由
黒い縁取りを追加することで：
- 色覚異常（色弱・色盲）の方でも形状のコントラストで認識可能
- 背景色と緑色の区別が難しい場合でも、黒い輪郭で明確に視認できる
- WCAGのコントラスト比基準をより確実に満たす

#### CSS実装
```css
.dropdown-item::before {
    content: '';
    display: inline-block;
    width: 0.75em;   /* Scaled relative to font size */
    height: 0.75em;  /* Scaled relative to font size */
    border-radius: 50%;
    border: 2px solid #666;
    background-color: transparent;
}

.dropdown-item.active::before {
    background-color: #4CAF50;
    border-color: #000;  /* Black border for color-blind users */
    border-width: 2px;
}
```

#### JavaScript実装
```javascript
if (langCode === currentLang) {
    item.classList.add('active');  // 選択された言語にactiveクラスを追加
}
```

### 2. フォーム入力フィールドのフォーカスインジケータ

#### 視覚的表現
- **フォーカス時**: フィールドの左側に大きな緑の塗りつぶし丸（●）+ 黒い縁取りを表示
- **ボーダー**: 緑色に変更
- **シャドウ**: 緑色の柔らかい影を追加

#### デザイン理由
黒い縁取りを追加することで：
- 色覚異常の方でも明確に認識可能
- 白やグレーの背景に対して常に高いコントラストを維持

#### CSS実装
```css
.form-group.active::before {
    content: '●';
    position: absolute;
    left: 0.5rem;
    bottom: 0.5rem;  /* Align with input field */
    color: #4CAF50;
    font-size: 1em;  /* Same as label - scales with user font size */
    line-height: 1;
    /* Black outline for color-blind users */
    text-shadow: 
        -1px -1px 0 #000,
         1px -1px 0 #000,
        -1px  1px 0 #000,
         1px  1px 0 #000,
        -1px  0   0 #000,
         1px  0   0 #000,
         0   -1px 0 #000,
         0    1px 0 #000;
}

.form-group label {
    padding-left: 1.5rem;  /* Indent label */
}

.form-group input {
    padding-left: 2.5rem;  /* Make space for indicator */
}

.form-group input:focus {
    border-color: #4CAF50;
    box-shadow: 0 0 0 3px rgba(76, 175, 80, 0.2);
}
```

#### JavaScript実装
```javascript
function setupAccessibilityIndicators() {
    document.querySelectorAll('input, textarea, select').forEach(input => {
        input.addEventListener('focus', function() {
            const formGroup = this.closest('.form-group');
            if (formGroup) {
                formGroup.classList.add('active');
            }
        });
        
        input.addEventListener('blur', function() {
            const formGroup = this.closest('.form-group');
            if (formGroup) {
                formGroup.classList.remove('active');
            }
        });
    });
}
```

### 3. ボタンのフォーカスインジケータ

#### 視覚的表現
- **フォーカス時**: ボタンの下部に濃い緑の下線を表示
- **アウトライン**: 3pxの緑のアウトラインを追加

#### CSS実装
```css
.btn-primary:focus {
    outline: 3px solid #4CAF50;
    outline-offset: 2px;
}

.btn-primary.focus-indicator:focus::after {
    content: '';
    position: absolute;
    bottom: -4px;
    left: 10%;
    right: 10%;
    height: 3px;
    background-color: #2e7d32;
    border-radius: 2px;
}
```

## 色の選択理由 (Color Choice Rationale)

### 緑色 (#4CAF50) を選択した理由
1. **コントラスト**: 白や薄いグレーの背景に対して高いコントラスト
2. **ポジティブな意味**: 「進む」「OK」「選択」を表す一般的な色
3. **アクセシビリティ**: WCAG 2.1のコントラスト比基準を満たす
4. **一貫性**: すでにボタンの主色として使用されている

### 黒い縁取りを追加した理由
1. **色覚バリアフリー**: 色に依存せず形状で認識可能
2. **コントラスト向上**: 背景色に関わらず常に明確な境界線
3. **視認性**: 緑と背景の区別が難しい場合でも黒い輪郭で認識可能
4. **WCAG準拠**: より確実にコントラスト比基準を満たす

### 色覚シミュレーション
以下の色覚タイプでもインジケータを認識可能：
- **1型色覚（P型、赤色弱）**: 黒い縁取りで認識
- **2型色覚（D型、緑色弱）**: 黒い縁取りで認識
- **3型色覚（T型、青色弱）**: 緑と黒のコントラストで認識
- **全色盲**: 輝度の違いと黒い縁取りで認識

### サイズの選択
- **丸のサイズ**: 0.75em（フォントサイズに追従、見やすいが邪魔にならない）
- **文字の丸**: 1em（ラベルと同じサイズ、フォントサイズに追従）
- **下線の太さ**: 3px（視認性が高い）

#### フォントサイズへの追従
すべてのインジケータのサイズは `em` 単位で指定されており、ユーザーがフォントサイズを変更した際に自動的に追従します。これにより、視力の弱い方がフォントサイズを大きくした場合でも、インジケータも同じように拡大され、判別しやすさが保たれます。

## 実装ファイル (Implementation Files)

### 変更したファイル
1. **res/css/menu.css**
   - インジケータのスタイル定義
   - フォーカス状態のスタイル
   - アクティブ状態のスタイル

2. **res/js/menu.js**
   - `setupLanguageMenu()` - activeクラスの追加
   - `setupAccessibilityIndicators()` - フォーカスリスナーの設定

## 使用方法 (Usage)

### 開発者向け

#### 新しいドロップダウンメニュー項目を追加する場合
```html
<div class="dropdown-item">項目名</div>
<div class="dropdown-item active">選択中の項目</div>
```

#### 新しいフォーム入力を追加する場合
```html
<div class="form-group">
    <label for="field-id">ラベル:</label>
    <input type="text" id="field-id" name="field-id" />
</div>
```

フォーカスインジケータは自動的に追加されます（`setupAccessibilityIndicators()`が実行されるため）。

#### 新しいボタンを追加する場合
```html
<button class="btn-primary focus-indicator">ボタン</button>
```

### ユーザー向け

#### 言語メニュー
- メニューを開くと、現在選択されている言語の前に**緑の塗りつぶしの丸（●）**が表示されます
- 他の言語は空の丸（○）が表示されます

#### フォーム入力
- フィールドにフォーカスすると、**左側に大きな緑の丸（●）**が表示されます
- フィールドの枠も緑色に変わります

#### ボタン
- ボタンにフォーカスすると、**下部に緑の下線**が表示されます
- 緑のアウトラインも表示されます

## 今後の拡張 (Future Enhancements)

### 計画中の機能
1. **カスタマイズ可能なインジケータ**
   - ユーザーが色やサイズを変更できる設定画面
   
2. **音声フィードバック**
   - スクリーンリーダー対応の強化
   - ARIA属性の追加

3. **キーボードナビゲーション**
   - すべての機能をキーボードのみで操作可能に
   - ショートカットキーの追加

4. **ハイコントラストモード**
   - より強いコントラストのテーマ
   - 白黒反転モード

5. **アニメーション**
   - インジケータの表示/非表示にフェードイン/アウト効果
   - 過度な動きは避け、motion-reduceメディアクエリに対応

## テスト (Testing)

### 手動テスト項目
- [ ] 言語メニューで選択言語に緑の丸（黒い縁取り付き）が表示される
- [ ] 言語を切り替えると丸が移動する
- [ ] フォーム入力にフォーカスすると緑の丸（黒い縁取り付き）が表示される
- [ ] フォーカスを外すと丸が消える
- [ ] ボタンにフォーカスすると下線が表示される
- [ ] Tabキーでフォーカスが正しく移動する

### アクセシビリティチェック
- [ ] キーボードのみで全機能が操作可能
- [ ] 色覚異常シミュレーションでも識別可能（黒い縁取りで判別）
- [ ] グレースケール表示でも識別可能
- [ ] スクリーンリーダーで読み上げ可能
- [ ] コントラスト比がWCAG 2.1 AAレベルを満たす

### 色覚異常対応チェック
- [ ] 1型色覚（P型、赤色弱）で緑の丸が認識できる
- [ ] 2型色覚（D型、緑色弱）で緑の丸が認識できる
- [ ] 3型色覚（T型、青色弱）で緑の丸が認識できる
- [ ] 全色盲（モノクロ視覚）で緑の丸が認識できる

## 参考資料 (References)

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WebAIM: Keyboard Accessibility](https://webaim.org/techniques/keyboard/)
- [Color Contrast Analyzer](https://developer.paciellogroup.com/resources/contrastanalyser/)
- [色覚バリアフリーガイドライン](https://jfly.uni-koeln.de/color/)
- [WebAIM: Color Blindness Simulator](https://www.color-blindness.com/coblis-color-blindness-simulator/)

## 変更履歴 (Change Log)

### 2024-10-24
- 初版作成
- 言語メニューのインジケータ実装
- フォーム入力のフォーカスインジケータ実装
- ボタンのフォーカスインジケータ実装
- インジケータサイズを `em` 単位に変更（フォントサイズに追従）
- すべてのインジケータがユーザーのフォントサイズ設定に追従するように改善
- 緑の丸に黒い縁取りを追加（色覚バリアフリー対応）
- 色に依存せず形状のコントラストで認識可能に改善
- インジケータ位置を入力フィールドの左側に修正（視覚的な関連性を向上）
