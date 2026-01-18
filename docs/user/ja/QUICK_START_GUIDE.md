# クイックスタートガイド

**バージョン**: 1.1.0
**最終更新**: 2026-01-18 JST

---

## はじめに

KakeiBon by Rustのインストールが完了していることを確認してください。未インストールの場合は、[インストールガイド](INSTALLATION_GUIDE.md)を参照してください。

---

## 5分でわかる！クイックスタート

### ステップ1: KakeiBon by Rustを起動

```bash
# ビルドディレクトリから実行
cd /path/to/KakeiBonByRust
./target/release/KakeiBon

# またはbinディレクトリにコピーした場合
./KakeiBon
```

**注意**: v1.0.1ではソースからのビルドが必要です。ビルド済みパッケージは今後のリリースで提供予定です。

### ステップ2: 管理者アカウントを作成

初回起動時、**管理者セットアップ画面**が表示されます：

1. **ユーザー名**を入力（3-20文字、英数字とアンダースコア）
   - 例: `admin`、`myname`、`user_admin`
2. **パスワード**を入力（最低16文字）
   - ヒント: 文字、数字、記号を組み合わせた強力なパスワードを使用
   - 例: `MySecurePass2024!@`
3. **「管理者を作成」**ボタンをクリック

✅ **これで管理者としてログインしました！**

### ステップ3: カテゴリを設定

カテゴリは収入と支出を整理するのに役立ちます。

**組み込みカテゴリ:**
KakeiBon by Rustにはデフォルトのカテゴリが用意されていますが、カスタマイズできます。

1. メニューから**「費目管理」**をクリック
2. 3階層のカテゴリ構造を確認：
   - レベル1: 大分類（例：「食費」「交通費」）
   - レベル2: 中分類（例：「食料品」「外食」）
   - レベル3: 小分類（オプション）

**新しいカテゴリを追加するには:**
1. 親カテゴリを選択（レベル1の場合は選択なし）
2. **日本語**と**英語**でカテゴリ名を入力
3. **「費目を追加」**をクリック

### ステップ4: 口座を追加

口座はお金がどこにあるか（銀行、現金、クレジットカードなど）を表します。

1. メニューから**「口座管理」**をクリック
2. **「口座を追加」**ボタンをクリック
3. 入力：
   - **口座名**: 例：「メイン銀行」「現金」「クレジットカード」
   - **口座種別**: 現金、銀行、クレジットカードなど
   - **初期残高**: 開始時の金額（オプション）
4. **「保存」**をクリック

[Idea] **ヒント**: 取引を記録する前に、少なくとも1つの口座を追加してください。

### ステップ5: 最初の取引を記録

1. メニューから**「入出金管理」**をクリック
2. **「入出金を追加」**ボタンをクリック
3. 入力：
   - **日付**: 取引日
   - **種別**: 収入または支出
   - **口座**: 使用する口座
   - **費目**: 所属するカテゴリ
   - **金額**: 取引金額
   - **説明**: メモ（オプション）
4. **「保存」**をクリック

✅ **おめでとうございます！最初の取引を記録しました！**

---

## 次のステップ

### 一般ユーザーを作成（管理者のみ）

複数の人がKakeiBon by Rustを使用する場合：

1. メニューから**「ユーザー管理」**をクリック
2. **「ユーザーを追加」**ボタンをクリック
3. ユーザー名とパスワードを入力
4. **「ユーザー」**ロール（管理者ではない）を割り当て
5. **「保存」**をクリック

[Idea] 一般ユーザーは取引を記録できますが、他のユーザーを管理できません。

### 集計を表示

支出パターンを分析：

1. メニューから**「集計」**をクリック
2. 集計タイプを選択：
   - **月次**: 月ごとの収入/支出を表示
   - **日次**: 特定の日付で表示
   - **週次**: 週ごとに表示
   - **年次**: 年ごとに表示
   - **期間**: カスタム日付範囲で表示
   - **ダッシュボード**: グラフで収支を可視化（v1.1.0新機能）
3. パラメータを選択（例：年と月）
4. **「集計」**をクリック
5. カテゴリ、口座、店舗別にグループ化された結果を表示

### ダッシュボードで可視化（v1.1.0新機能）

グラフで収支を視覚的に把握：

1. メニューから**「集計」→「ダッシュボード」**をクリック
2. 3種類のグラフが表示されます：
   - **円グラフ**: カテゴリ別支出内訳
   - **棒グラフ**: 月次収支比較
   - **折れ線グラフ**: 月次トレンド

[Idea] **ヒント**: ダッシュボードは月初に確認すると、先月の収支状況を振り返るのに便利です

### カテゴリをカスタマイズ

ライフスタイルに合ったカテゴリ構造を構築：

1. **詳細なサブカテゴリを追加**
   - 例: 食費 → 食料品 → 野菜
   - 例: 交通費 → 公共交通機関 → 電車

2. **未使用カテゴリを無効化**
   - 削除ではなく「無効」にマーク
   - 過去のデータを保持

### 店舗を設定（オプション）

よく利用する店舗を追跡：

1. **「店舗管理」**をクリック（利用可能な場合）
2. 定期的に訪れる店舗を追加
3. 取引記録時に店舗を割り当て
4. 集計で店舗別の支出を確認

---

## 効果的な使い方のコツ

### [Target] 毎日の習慣
- **すぐに取引を記録** - 一日の終わりまで待たない
- **わかりやすいメモを使用** - 未来の自分が感謝します
- **定期的に残高をチェック** - 口座の正確性を保つ

### [Chart] 月次レビュー
- **月次集計を実行** - お金の使い道を確認
- **前月と比較** - トレンドを発見
- **カテゴリを調整** - 必要に応じて追加/修正

### [Key] セキュリティのベストプラクティス
- **強力なパスワードを使用**（16文字以上）
- **管理者アカウントを共有しない** - 個別のユーザーを作成
- **定期的にデータベースをバックアップ**（以下参照）

### [Save] データのバックアップ

データは`.kakeibon`ディレクトリに保存されています：

**Linux/macOS:**
- データベース: `~/.kakeibon/KakeiBonDB.sqlite3`
- 設定ファイル: `~/.kakeibon/KakeiBon.json`

**Windows:**
- データベース: `%USERPROFILE%\.kakeibon\KakeiBonDB.sqlite3`
- 設定ファイル: `%USERPROFILE%\.kakeibon\KakeiBon.json`

**定期的にバックアップ:**
```bash
# Linux/macOS - ディレクトリ全体をバックアップ
tar -czf ~/backups/kakeibon_backup_$(date +%Y%m%d).tar.gz ~/.kakeibon/

# または個別にバックアップ
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backups/kakeibon_db_$(date +%Y%m%d).sqlite3
cp ~/.kakeibon/KakeiBon.json ~/backups/kakeibon_config_$(date +%Y%m%d).json

# Windows (PowerShell) - ディレクトリ全体をバックアップ
Compress-Archive -Path "$env:USERPROFILE\.kakeibon" -DestinationPath "$env:USERPROFILE\backups\kakeibon_backup_$(Get-Date -Format 'yyyyMMdd').zip"

# または個別にバックアップ
Copy-Item "$env:USERPROFILE\.kakeibon\KakeiBonDB.sqlite3" "$env:USERPROFILE\backups\kakeibon_db_$(Get-Date -Format 'yyyyMMdd').sqlite3"
Copy-Item "$env:USERPROFILE\.kakeibon\KakeiBon.json" "$env:USERPROFILE\backups\kakeibon_config_$(Get-Date -Format 'yyyyMMdd').json"
```

**バックアップからの復元:**
```bash
# Linux/macOS - ディレクトリ全体を復元
tar -xzf ~/backups/kakeibon_backup_20251203.tar.gz -C ~/

# または個別に復元
cp ~/backups/kakeibon_db_20251203.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3
cp ~/backups/kakeibon_config_20251203.json ~/.kakeibon/KakeiBon.json

# Windows (PowerShell) - ディレクトリ全体を復元
Expand-Archive -Path "$env:USERPROFILE\backups\kakeibon_backup_20251203.zip" -DestinationPath "$env:USERPROFILE" -Force

# または個別に復元
Copy-Item "$env:USERPROFILE\backups\kakeibon_db_20251203.sqlite3" "$env:USERPROFILE\.kakeibon\KakeiBonDB.sqlite3"
Copy-Item "$env:USERPROFILE\backups\kakeibon_config_20251203.json" "$env:USERPROFILE\.kakeibon\KakeiBon.json"
```

**[Idea] ヒント:** 設定ファイル（`KakeiBon.json`）には言語やフォントサイズなどの設定が保存されています。バックアップすることで設定を失わずに済みます。

---

## よくある質問

### Q: 言語を変更できますか？
**A:** はい！言語メニュー（[Globe]）をクリックして、日本語または英語（English）を選択してください。

### Q: パスワードを忘れました。どうすればいいですか？
**A:** 現在、パスワードリセット機能はありません。パスワードは安全に保管してください！

### Q: 複数の人が同時にKakeiBon by Rustを使用できますか？
**A:** KakeiBon by Rustはローカル使用向けに設計されています。複数のユーザーがアカウントを持つことはできますが、同じコンピュータで一度に一人ずつ使用してください。

### Q: 他のアプリからデータをインポートできますか？
**A:** インポート機能は将来のリリースで予定されています。現在は手動入力が必要です。

### Q: 取引リストが長くなりました。フィルタリングするには？
**A:** 入出金管理のフィルタオプションを使用して、日付範囲、カテゴリ、口座で表示できます。

---

## ヘルプを得る

### ドキュメント
- [インストールガイド](INSTALLATION_GUIDE.md) - セットアップ手順
- [トラブルシューティングガイド](TROUBLESHOOTING.md) - 一般的な問題
- [集計機能ユーザーガイド](AGGREGATION_USER_GUIDE.md) - 詳細な集計機能

### サポート
- **GitHub Issues**: [バグ報告や機能リクエスト](https://github.com/BonoJovi/KakeiBonByRust/issues)
- **Discussions**: [質問やアイデアの共有](https://github.com/BonoJovi/KakeiBonByRust/discussions)

---

## 準備完了です！ [Party]

これでKakeiBon by Rustの基本がわかりました。取引の記録を始めて、財務の可視化を実現しましょう！

ハッピーな家計管理を！ [Money][Chart]

---

**次へ**: 高度な分析機能については[集計機能ユーザーガイド](AGGREGATION_USER_GUIDE.md)をご覧ください。
