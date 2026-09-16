<template>
    <section class="panel">
        <header class="panel-head">
            <h2>推送</h2>
            <p>Note4 是 4.2″ 400×300 黑白屏 —— 推图片时建议用高对比度线稿，彩色照片靠抖动算法转成黑白后细节会丢失。</p>
        </header>

        <div class="card">
            <div class="card-head">
                <h3>目标设备</h3>
                <span v-if="targetDeviceId" class="badge badge-ok">{{ deviceLabel(selectedDevice) }}</span>
                <span v-else class="badge badge-warn">未选择</span>
            </div>

            <p v-if="devices.length === 0" class="hint">还没有设备。先到「设备」页刷新一次，确认账号下已绑定 Note4。</p>

            <div v-else class="field-row">
                <select v-model="selectedDeviceId" class="inline-select">
                    <option v-for="device in devices" :key="device.deviceId" :value="device.deviceId">
                        {{ deviceLabel(device) }}（{{ device.deviceId }}）
                    </option>
                </select>

                <label class="inline-field">
                    <span>页面</span>
                    <select v-model="pageId">
                        <option value="">不指定（临时刷新）</option>
                        <option v-for="id in PAGE_IDS" :key="id" :value="id">页面 {{ id }}</option>
                    </select>
                </label>
            </div>

            <p class="hint">指定页面会<strong>持久化存储</strong>到设备；不指定则只做一次临时刷新。</p>
        </div>

        <div class="segmented">
            <button
                v-for="item in MODES"
                :key="item.key"
                class="seg-btn"
                :class="{ active: mode === item.key }"
                @click="
                    mode = item.key;
                    resetFeedback();
                "
            >
                {{ item.label }}
            </button>
        </div>

        <div class="card">
            <template v-if="mode === 'text'">
                <div class="card-head"><h3>推送纯文本</h3></div>
                <textarea v-model="textForm.text" rows="5" placeholder="今日天气：晴&#10;温度：25°C"></textarea>
                <label class="inline-field">
                    <span>字号（0 = 设备默认）</span>
                    <input v-model.number="textForm.fontSize" type="number" min="0" max="72" />
                </label>
                <div class="field-row">
                    <button class="btn btn-primary" :disabled="!canPush" @click="onPushText">
                        {{ busy ? "推送中…" : "推送文本" }}
                    </button>
                </div>
            </template>

            <template v-else-if="mode === 'structured'">
                <div class="card-head"><h3>推送标题 + 正文</h3></div>
                <label>
                    <span>标题</span>
                    <input v-model="structuredForm.title" placeholder="会议提醒" />
                </label>
                <label>
                    <span>正文</span>
                    <textarea
                        v-model="structuredForm.body"
                        rows="4"
                        placeholder="15:00 三楼会议室&#10;请带笔记本"
                    ></textarea>
                </label>
                <div class="field-row">
                    <button class="btn btn-primary" :disabled="!canPush" @click="onPushStructured">
                        {{ busy ? "推送中…" : "推送标题 + 正文" }}
                    </button>
                </div>
            </template>

            <template v-else>
                <div class="card-head">
                    <h3>推送图片</h3>
                    <span class="hint">{{ picked.length }} / {{ MAX_IMAGES }} 张</span>
                </div>

                <div class="field-row">
                    <label class="btn btn-file">
                        <input type="file" accept="image/*" multiple @change="onPickFiles" />
                        {{ picking ? "读取中…" : "选择图片" }}
                    </label>
                    <button class="btn btn-small" :disabled="picked.length === 0" @click="revokeAll">清空选择</button>
                    <label class="inline-check">
                        <input v-model="dither" type="checkbox" />
                        <span>使用抖动算法（关闭则硬阈值二值化）</span>
                    </label>
                </div>

                <ul v-if="picked.length > 0" class="thumb-list">
                    <li v-for="(item, index) in picked" :key="item.previewUrl">
                        <img :src="item.previewUrl" :alt="item.upload.name" />
                        <div class="thumb-meta">
                            <span class="thumb-name">{{ item.upload.name }}</span>
                            <span class="hint">{{ formatSize(item.size) }}</span>
                        </div>
                        <button class="btn btn-small" @click="removeImage(index)">移除</button>
                    </li>
                </ul>

                <div class="field-row">
                    <button class="btn btn-primary" :disabled="!canPush || picked.length === 0" @click="onPushImage">
                        {{ busy ? "推送中…" : `推送 ${picked.length} 张图片` }}
                    </button>
                </div>
            </template>
        </div>

        <div class="card danger-zone">
            <div class="card-head"><h3>清空页面</h3></div>
            <p class="hint">
                {{ pageId ? `删除设备的页面 ${pageId}` : "删除该设备的全部页面（不可恢复）" }}
            </p>
            <div class="field-row">
                <button
                    class="btn"
                    :class="confirmClear ? 'btn-danger' : ''"
                    :disabled="!canPush"
                    @click="onClearPages"
                >
                    {{ confirmClear ? "再点一次确认清空" : pageId ? `清空页面 ${pageId}` : "清空全部页面" }}
                </button>
                <button v-if="confirmClear" class="btn btn-small" @click="confirmClear = false">取消</button>
            </div>
        </div>

        <p v-if="error" class="feedback feedback-err">{{ error }}</p>
        <p v-else-if="notice" class="feedback feedback-ok">{{ notice }}</p>
    </section>
</template>

<script setup lang="ts">
import {
    clearPages,
    fileToImageUpload,
    pushImage,
    pushStructuredText,
    pushText,
    ZectrixApiError,
} from "../api/zectrix";
import { useDevices } from "../composables/useDevices";
import { deviceLabel, PAGE_IDS, type ImageUpload } from "../types/zectrix";

const { devices, selectedDeviceId, selectedDevice, refresh: refreshDevices } = useDevices();

type Mode = "text" | "structured" | "image";

const MODES: { key: Mode; label: string }[] = [
    { key: "text", label: "纯文本" },
    { key: "structured", label: "标题 + 正文" },
    { key: "image", label: "图片" },
];

/** 云端限制：单次 ≤5 张，单张 ≤2 MB */
const MAX_IMAGES = 5;
const MAX_IMAGE_BYTES = 2 * 1024 * 1024;

interface PickedImage {
    upload: ImageUpload;
    size: number;
    previewUrl: string;
}

const mode = ref<Mode>("text");
const pageId = ref("");
const dither = ref(true);
const busy = ref(false);
const error = ref<string | null>(null);
const notice = ref<string | null>(null);
const confirmClear = ref(false);

const textForm = reactive({ text: "", fontSize: 0 });
const structuredForm = reactive({ title: "", body: "" });
const picked = ref<PickedImage[]>([]);
const picking = ref(false);

const targetDeviceId = computed(() => selectedDevice.value?.deviceId ?? "");
const canPush = computed(() => Boolean(targetDeviceId.value) && !busy.value);

function describe(error: unknown): string {
    return error instanceof ZectrixApiError ? error.message : String(error);
}

function resetFeedback() {
    error.value = null;
    notice.value = null;
    confirmClear.value = false;
}

function formatSize(bytes: number): string {
    return bytes >= 1024 * 1024 ? `${(bytes / 1024 / 1024).toFixed(2)} MB` : `${Math.round(bytes / 1024)} KB`;
}

function revokeAll() {
    picked.value.forEach((item) => URL.revokeObjectURL(item.previewUrl));
    picked.value = [];
}

async function onPickFiles(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = Array.from(input.files ?? []);
    input.value = "";
    if (files.length === 0) return;

    resetFeedback();
    picking.value = files.length > 0;

    try {
        for (const file of files) {
            if (picked.value.length >= MAX_IMAGES) {
                error.value = `最多 ${MAX_IMAGES} 张，多出的已忽略`;
                break;
            }
            if (!file.type.startsWith("image/")) {
                error.value = `「${file.name}」不是图片，已跳过`;
                continue;
            }
            if (file.size > MAX_IMAGE_BYTES) {
                error.value = `「${file.name}」${formatSize(file.size)}，超过单张 2 MB 上限，已跳过`;
                continue;
            }
            picked.value.push({
                upload: await fileToImageUpload(file),
                size: file.size,
                previewUrl: URL.createObjectURL(file),
            });
        }
    } finally {
        picking.value = false;
    }
}

function removeImage(index: number) {
    const [removed] = picked.value.splice(index, 1);
    if (removed) URL.revokeObjectURL(removed.previewUrl);
}

async function run(action: () => Promise<string | void>, fallback: string) {
    error.value = null;
    notice.value = null;
    busy.value = true;
    try {
        const custom = await action();
        notice.value = typeof custom === "string" ? custom : fallback;
    } catch (e) {
        error.value = describe(e);
    } finally {
        busy.value = false;
    }
}

const onPushText = () =>
    run(
        () =>
            pushText({
                deviceId: targetDeviceId.value,
                text: textForm.text,
                fontSize: textForm.fontSize > 0 ? textForm.fontSize : undefined,
                pageId: pageId.value || undefined,
            }),
        "文本已推送",
    );

const onPushStructured = () =>
    run(
        () =>
            pushStructuredText({
                deviceId: targetDeviceId.value,
                title: structuredForm.title,
                body: structuredForm.body,
                pageId: pageId.value || undefined,
            }),
        "标题 + 正文已推送",
    );

const onPushImage = () =>
    run(async () => {
        const result = await pushImage({
            deviceId: targetDeviceId.value,
            images: picked.value.map((item) => item.upload),
            dither: dither.value,
            pageId: pageId.value || undefined,
        });
        if (result?.pushedPages != null) {
            return `已推送 ${result.pushedPages} 页（共 ${result.totalPages ?? "?"} 页）`;
        }
        return "图片已推送";
    }, "图片已推送");

async function onClearPages() {
    // 两步确认：第一次点击只切换按钮文案，不做任何请求
    if (!confirmClear.value) {
        error.value = null;
        notice.value = null;
        confirmClear.value = true;
        return;
    }
    confirmClear.value = false;
    await run(
        () =>
            clearPages({
                deviceId: targetDeviceId.value,
                pageId: pageId.value || undefined,
            }),
        pageId.value ? `页面 ${pageId.value} 已清空` : "该设备全部页面已清空",
    );
}

onMounted(async () => {
    try {
        await refreshDevices();
    } catch {
        // 未配置 Key 时由各操作按钮统一提示
    }
});

onBeforeUnmount(revokeAll);
</script>
