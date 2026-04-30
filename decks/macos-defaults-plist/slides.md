---
marp: true
theme: marp-pj-default
paginate: true
header: ''
footer: ''
---

<!-- _class: title-slide -->

# macOS defaults<br>plist 運用の実践

<div class="my-name">林 明虎</div>
<p class="talk-meta">2026/4/30 夕会 LT会</p>

---

# 今日のゴール

- `defaults` と `.plist` の役割分担を理解する
- 直接編集が危険な理由を説明できるようにする
- チームで使える運用ルールに落とす

---

# 前提: 全キー一覧は存在しない

- Apple 公式の「全 defaults キー一覧」はない
- 設定キーは OS / アプリ更新で変動する
- 固定リストより「探索手順」の整備が重要

---

# 設定を探す基本コマンド

```bash
defaults domains
defaults read com.apple.finder
```

- `domains` で存在ドメインを列挙
- `read` で現在値を確認
- 外部チートシートは補助、最終判断はローカル検証

---

# 設定保存の実態

- 設定ファイルは初回起動や設定変更時に作られる
- 未設定項目はアプリ内デフォルト値が使われる
- 実体は `~/Library/Preferences/com.vendor.app.plist`
- 独自ドメインの作成も可能（命名衝突は回避必須）

---

# なぜ plist が読めないのか

- 多くの `.plist` はバイナリ形式
- 通常エディタでは可読性が低い
- 可読化には `plutil` を使う

```bash
plutil -convert xml1 target.plist
plutil -convert binary1 target.plist
plutil -convert xml1 target.plist
plutil -convert binary1 target.plist
```

---

# 直接編集のリスク

> 直接編集は最後の手段

<div style="height: 64px;"></div>

- `cfprefsd` が設定をキャッシュしている
- ファイル直編集だけでは反映されないことがある
- 古いキャッシュで上書きされる可能性がある

---

# 推奨運用

- 設定変更は原則 `defaults`
- 直接編集は例外対応に限定
- 例外時はキャッシュ整合を明示的に行う

```bash
killall cfprefsd
```

---

# 結論

- 「値の場所」より「変更経路」が重要
- 安全経路は `defaults`、直接編集は最後の手段
- チーム標準として運用ルールを明文化する
