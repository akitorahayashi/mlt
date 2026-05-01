---
marp: true
theme: mlt-default
paginate: true
header: ''
footer: ''
---

<!-- _class: title-slide -->

# macOS の設定は<br>覚えるより<br>defaults で再現しよう

<div class="my-name">林 明虎</div>
<p class="talk-meta">2026/4/30 夕会 LT会</p>

---

# 目次

- `defaults` で何ができるか
- `defaults`のドメインと `.plist`
- 安全に設定を探して反映する流れ
- よく使う設定をまとめてスクリプト化する

---

# `defaults` で何ができるか

```sh
defaults read com.apple.finder
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true
```

<div style="height: 40px;"></div>

- macOS の設定を読む、書くための標準コマンド
- 設定アプリを毎回開かずに操作できる
- 複数端末の設定をそろえるのに向いている

---

# ドメイン

<p style="margin: 32px;">ドメイン は 設定を管理する単位</p>

- `com.apple.dock` は Dock の設定
- `com.apple.finder` は Finder の設定
- `NSGlobalDomain` はユーザ全体に効く設定

---

# 設定ファイルの実体

- 設定は `.plist` ファイルとして保存される
- 多くの場合はバイナリ形式で直接読みにくい
- 直接編集より `defaults` を使うほうが安全

```text
~/Library/Preferences/com.apple.finder.plist # ユーザレベル
/Library/Preferences/ # この領域の書き換えには管理者権限（sudo）が必要
~/Library/Containers/.../Data/Library/Preferences/ # アプリレベル
```

---

# 直接編集が危ない理由

- macOS は `cfprefsd` で設定をキャッシュしている
  → ファイルを手で書き換えても反映されないことがある
- `defaults write` はキャッシュ更新まで面倒を見る

---

# どうやって探すか

```sh
defaults read com.apple.dock
defaults write com.apple.dock tilesize -int 30
```

- まず `defaults read` で現在値を確認する
- 値がありそうなら `defaults write` で反映する
- 全キー一覧はないので、検索と検証を組み合わせる

---

# 実行方法

```sh
defaults write [ドメイン名] [キー] -[型] [値]
```

<div style="text-align: center;">具体例</div>

```sh
defaults write com.apple.dock tilesize -int 30
killall Dock
```

```sh
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true
killall Finder
```

---

<h1 style="top: 48px;">シェルスクリプトにまとめる</h1>

```sh
#!/usr/bin/env bash
set -euo pipefail

defaults write com.apple.dock tilesize -int 30
defaults write com.apple.finder _FXShowPosixPathInTitle -bool true

killall Dock
killall Finder
```

- よく使う設定は 1 回で流せる形にしておく
- 初期化後の復元や複数端末への展開が楽になる
- 変更履歴を git で管理

---

# まとめ

- `defaults` を使うと、設定アプリで毎回探さなくても設定をコマンドで再現できる
- macbook や mac mini など複数の macOS デバイスで設定を統一できる
- 新しい Mac やクリーンインストール後でも、同じ設定を短時間で戻せる
