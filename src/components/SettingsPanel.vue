<template>
    <section class="panel">
        <header class="panel-head">
            <h2>设置</h2>
            <p>API Key 只保存在本机系统凭据管理器，前端无法读取，也不会写入仓库。</p>
        </header>

        <div class="card">
            <div class="card-head">
                <h3>API Key</h3>
                <span class="badge" :class="status.configured ? 'badge-ok' : 'badge-warn'">
                    {{ status.configured ? "已配置" : "未配置" }}
                </span>
            </div>

            <p v-if="status.configured" class="masked-key">
                当前：<code>{{ status.masked }}</code>
            </p>
            <p v-else class="hint">
                还没有配置 Key。到
                <a href="https://cloud.zectrix.com" target="_blank" rel="noreferrer">cloud.zectrix.com</a>
                扫码登录后，进入「开放 API → 创建 API Key」，把生成的 <code>zt_</code> 开头的字符串粘贴到下面。
            </p>

            <div class="field-row">
                <input
                    v-model="draftKey"
                    type="password"
                    autocomplete="off"
                    spellcheck="false"
                    :placeholder="status.configured ? '粘贴新 Key 以替换' : 'zt_xxxxxxxxxxxxxxxx'"
                    @keyup.enter="onSave"
                />
                <button class="btn btn-primary" :disabled="busy" @click="onSave">保存</button>
            </div>

            <div class="field-row">
                <button class="btn" :disabled="busy || !status.configured" @click="onVerify">测试连接</button>
                <button class="btn btn-danger" :disabled="busy || !status.configured" @click="onClear">清除 Key</button>
            </div>

            <p v-if="message" class="feedback" :class="message.tone === 'ok' ? 'feedback-ok' : 'feedback-err'">
                {{ message.text }}
            </p>
        </div>

        <div class="card">
            <div class="card-head"><h3>关于</h3></div>
            <p class="hint">
                基于 Tauri 2 + Vue 3 + Rust 构建的 Zectrix Note4 桌面管理端，通过极趣云开放 API
                管理设备、待办与屏幕内容推送。
            </p>
            <div class="logo-row">
                <img :src="tauriLogo" alt="Tauri" />
                <img :src="vueLogo" alt="Vue" />
                <img :src="viteLogo" alt="Vite" />
            </div>
            <dl class="kv">
                <dt>API 基础地址</dt>
                <dd><code>https://cloud.zectrix.com/open/v1</code></dd>
                <dt>最新文档</dt>
                <dd>
                    <a href="https://cloud.zectrix.com/home/api-docs" target="_blank" rel="noreferrer">
                        在线 API 文档
                    </a>
                </dd>
            </dl>
        </div>
    </section>
</template>

<script setup lang="ts">
import { ZectrixApiError } from "../api/zectrix";
import tauriLogo from "../assets/tauri.svg";
import viteLogo from "../assets/vite.svg";
import vueLogo from "../assets/vue.svg";
import { useApiKey } from "../composables/useApiKey";

const { status, save, clear, verify } = useApiKey();

const draftKey = ref("");
const busy = ref(false);
const message = ref<{ tone: "ok" | "err"; text: string } | null>(null);

function report(tone: "ok" | "err", text: string) {
    message.value = { tone, text };
}

function describe(error: unknown): string {
    return error instanceof ZectrixApiError ? error.message : String(error);
}

async function onSave() {
    if (!draftKey.value.trim()) {
        report("err", "请先粘贴 API Key");
        return;
    }
    busy.value = true;
    try {
        await save(draftKey.value);
        draftKey.value = "";
        report("ok", "API Key 已写入系统凭据管理器（DPAPI 加密）");
    } catch (error) {
        report("err", describe(error));
    } finally {
        busy.value = false;
    }
}

async function onVerify() {
    busy.value = true;
    try {
        const devices = await verify();
        report("ok", `连接正常，读到 ${devices.length} 台设备`);
    } catch (error) {
        report("err", describe(error));
    } finally {
        busy.value = false;
    }
}

async function onClear() {
    busy.value = true;
    try {
        await clear();
        report("ok", "API Key 已清除");
    } catch (error) {
        report("err", describe(error));
    } finally {
        busy.value = false;
    }
}
</script>
