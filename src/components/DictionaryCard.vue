<template>
  <div class="dictionary-card">
    <div class="card-inner">
      <div class="header">
        <div class="header-left">
          <span class="header-icon">📖</span>
          <span class="header-word">{{ data.word }}</span>
        </div>
        <div class="header-actions">
          <span class="shortcut-hint"><Kbd>Tab</Kbd> 切换翻译语言</span>
          <span class="shortcut-hint"><Kbd>↵</Kbd> 复制选中</span>
          <span class="shortcut-hint"><Kbd>D</Kbd> 查看详情</span>
        </div>
      </div>

      <div class="translation-title" @click="expanded = !expanded">
        <span>{{ data.translation }}</span>
        <svg
          class="chevron"
          :class="{ 'chevron-up': expanded }"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="6 9 12 15 18 9" />
        </svg>
      </div>

      <Transition name="expand">
        <div v-show="expanded" class="detail-card">
          <div class="word-header">
            <span class="word-text">{{ data.word }}</span>
            <span class="phonetic">/{{ data.phonetic }}/</span>
            <button class="speak-btn" @click="handleSpeak">
              <svg
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
                <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
                <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
              </svg>
            </button>
          </div>

          <div class="tags">
            <span v-for="tag in data.tags" :key="tag" class="tag" :class="tagClass(tag)">
              {{ tag }}
            </span>
          </div>

          <div class="definitions">
            <div v-for="def in data.definitions" :key="def.pos" class="def-row">
              <span class="def-pos">{{ def.pos }}</span>
              <span class="def-text">{{ def.text }}</span>
            </div>
          </div>

          <div class="divider" />

          <div class="section">
            <h4 class="section-title">词形变化</h4>
            <div class="word-forms">
              <span v-for="(val, key) in data.forms" :key="key" class="word-form">
                <span class="word-form-label">{{ key }}:</span>
                <span class="word-form-value">{{ val }}</span>
              </span>
            </div>
          </div>

          <div class="divider" />

          <div class="section">
            <h4 class="section-title">例句</h4>
            <ul class="examples">
              <li v-for="(ex, i) in data.examples" :key="i" class="example-item">
                <p class="example-en">{{ ex.en }}</p>
                <p class="example-cn">{{ ex.cn }}</p>
              </li>
            </ul>
            <div class="more-link">
              <a href="javascript:void(0)">查看更多例句 ›</a>
            </div>
          </div>

          <div class="divider" />

          <div class="section">
            <h4 class="section-title">近义词</h4>
            <div class="word-tags">
              <span v-for="w in data.synonyms" :key="w" class="word-tag">{{ w }}</span>
            </div>
          </div>

          <div class="section">
            <h4 class="section-title">反义词</h4>
            <div class="word-tags">
              <span v-for="w in data.antonyms" :key="w" class="word-tag">{{ w }}</span>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

export interface DictionaryData {
  word: string;
  translation: string;
  phonetic: string;
  tags: string[];
  definitions: { pos: string; text: string }[];
  forms: Record<string, string>;
  examples: { en: string; cn: string }[];
  synonyms: string[];
  antonyms: string[];
}

interface Props {
  data: DictionaryData;
}

const props = defineProps<Props>();
const expanded = ref(true);

function tagClass(tag: string) {
  const map: Record<string, string> = {
    CET4: "tag-cet4",
    CET6: "tag-cet6",
    考研: "tag-kaoyan",
    IELTS: "tag-ielts",
    TOEFL: "tag-toefl",
  };
  return map[tag] || "tag-default";
}

function handleSpeak() {
  if ("speechSynthesis" in window) {
    const utterance = new SpeechSynthesisUtterance(props.data.word);
    utterance.lang = "en-US";
    speechSynthesis.speak(utterance);
  }
}
</script>

<style scoped>
.dictionary-card {
  background: var(--secondary, oklch(0.96 0.003 250));
  border-radius: 12px;
  padding: 8px;
}

.card-inner {
  background: var(--card, oklch(1 0 0));
  border-radius: 10px;
  overflow: hidden;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-icon {
  font-size: 20px;
}

.header-word {
  font-size: 20px;
  font-weight: 600;
  color: var(--foreground, oklch(0.15 0.005 250));
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.shortcut-hint {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--muted-foreground, oklch(0.48 0 0));
  user-select: none;
}

.translation-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  font-size: 20px;
  font-weight: 600;
  color: var(--foreground, oklch(0.15 0.005 250));
  cursor: pointer;
  user-select: none;
}

.detail-card {
  margin: 0 8px 8px;
  padding: 20px;
  background: var(--card, oklch(1 0 0));
  border: 1px solid var(--border, oklch(0.89 0.005 250));
  border-radius: 8px;
}

.word-header {
  display: flex;
  align-items: baseline;
  gap: 8px;
  margin-bottom: 10px;
}

.word-text {
  font-size: 24px;
  font-weight: 700;
  color: var(--foreground, oklch(0.15 0.005 250));
}

.phonetic {
  font-size: 14px;
  color: var(--muted-foreground, oklch(0.48 0 0));
}

.speak-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  background: none;
  color: var(--muted-foreground, oklch(0.48 0 0));
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.15s;
}

.speak-btn:hover {
  background: var(--secondary, oklch(0.96 0.003 250));
  color: var(--foreground, oklch(0.15 0.005 250));
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 16px;
}

.tag {
  display: inline-block;
  padding: 2px 8px;
  font-size: 11px;
  font-weight: 500;
  border-radius: 4px;
  letter-spacing: 0.02em;
}

.tag-cet4 {
  color: #0ea5e9;
  background: #e0f2fe;
  border: 1px solid #bae6fd;
}

.tag-cet6 {
  color: #8b5cf6;
  background: #ede9fe;
  border: 1px solid #ddd6fe;
}

.tag-kaoyan {
  color: #059669;
  background: #d1fae5;
  border: 1px solid #a7f3d0;
}

.tag-ielts {
  color: #d97706;
  background: #fef3c7;
  border: 1px solid #fde68a;
}

.tag-toefl {
  color: #dc2626;
  background: #fee2e2;
  border: 1px solid #fecaca;
}

.tag-default {
  color: var(--muted-foreground);
  background: var(--secondary);
  border: 1px solid var(--border);
}

.definitions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.def-row {
  display: flex;
  align-items: baseline;
  gap: 16px;
}

.def-pos {
  flex-shrink: 0;
  width: 36px;
  font-size: 14px;
  font-weight: 600;
  color: var(--foreground, oklch(0.15 0.005 250));
}

.def-text {
  font-size: 14px;
  line-height: 1.6;
  color: var(--foreground, oklch(0.15 0.005 250));
}

.divider {
  height: 1px;
  margin: 16px 0;
  background: var(--border, oklch(0.89 0.005 250));
}

.section {
  margin-bottom: 4px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--foreground, oklch(0.15 0.005 250));
  margin-bottom: 8px;
}

.word-forms {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.word-form {
  font-size: 13px;
}

.word-form-label {
  color: var(--muted-foreground, oklch(0.48 0 0));
}

.word-form-value {
  color: var(--foreground, oklch(0.15 0.005 250));
  margin-left: 4px;
}

.examples {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.example-item {
  padding-left: 12px;
  border-left: 2px solid var(--border, oklch(0.89 0.005 250));
}

.example-en {
  font-size: 14px;
  color: var(--foreground, oklch(0.15 0.005 250));
  line-height: 1.5;
}

.example-cn {
  font-size: 13px;
  color: var(--muted-foreground, oklch(0.48 0 0));
  line-height: 1.5;
}

.more-link {
  text-align: right;
  margin-top: 8px;
}

.more-link a {
  font-size: 13px;
  color: var(--primary, oklch(0.62 0.18 255));
  text-decoration: none;
}

.more-link a:hover {
  text-decoration: underline;
}

.word-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.word-tag {
  display: inline-block;
  padding: 4px 12px;
  font-size: 13px;
  color: var(--foreground, oklch(0.15 0.005 250));
  background: var(--secondary, oklch(0.96 0.003 250));
  border: 1px solid var(--border, oklch(0.89 0.005 250));
  border-radius: 6px;
  cursor: default;
  transition: background 0.15s;
}

.word-tag:hover {
  background: var(--accent, oklch(0.93 0.01 250));
}

.expand-enter-active,
.expand-leave-active {
  transition: all 0.25s ease;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
  padding-top: 0;
  padding-bottom: 0;
}

.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 800px;
}

.chevron {
  transition: transform 0.2s ease;
  color: var(--muted-foreground, oklch(0.48 0 0));
}

.chevron-up {
  transform: rotate(180deg);
}

.shortcut-hint :deep(kbd) {
  color: var(--foreground, oklch(0.15 0.005 250));
  background: var(--secondary, oklch(0.96 0.003 250));
  border-color: var(--border, oklch(0.89 0.005 250));
  box-shadow: none;
}
</style>
