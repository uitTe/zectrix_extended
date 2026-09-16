<template>
    <div class="app-shell">
        <aside class="sidebar">
            <div class="brand">
                <span class="brand-mark">Zx</span>
                <div class="brand-text">
                    <strong>zectrix-extended</strong>
                    <small>Note4 桌面管理端</small>
                </div>
            </div>

            <nav class="nav">
                <button
                    v-for="tab in TABS"
                    :key="tab.key"
                    class="nav-item"
                    :class="{ active: active === tab.key }"
                    @click="active = tab.key"
                >
                    <span class="nav-label">{{ tab.label }}</span>
                    <small class="nav-hint">{{ tab.hint }}</small>
                </button>
            </nav>

            <button class="key-chip" :class="status.configured ? 'ok' : 'warn'" @click="active = 'settings'">
                <span class="dot"></span>
                <span class="key-chip-text">{{ status.configured ? status.masked : "API Key 未配置" }}</span>
            </button>
        </aside>

        <main class="content">
            <div v-if="!status.configured" class="banner">
                还没配置极趣云 API Key，设备与推送都无法使用。
                <button class="link-btn" @click="active = 'settings'">去设置</button>
            </div>

            <KeepAlive>
                <component :is="currentPanel" />
            </KeepAlive>
        </main>
    </div>
</template>

<script setup lang="ts">
import DevicePanel from "./components/DevicePanel.vue";
import PushPanel from "./components/PushPanel.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import TodoPanel from "./components/TodoPanel.vue";
import { useApiKey } from "./composables/useApiKey";

type TabKey = "devices" | "todos" | "push" | "settings";

const TABS: { key: TabKey; label: string; hint: string }[] = [
    { key: "devices", label: "设备", hint: "已绑定的 Note4" },
    { key: "todos", label: "待办", hint: "云端待办管理" },
    { key: "push", label: "推送", hint: "内容推到设备屏" },
    { key: "settings", label: "设置", hint: "API Key 与关于" },
];

/** 面板组件表：key 与 TABS 一一对应；交给 KeepAlive 按组件缓存实例，切 Tab 不再丢本地状态 */
const PANELS = {
    devices: DevicePanel,
    todos: TodoPanel,
    push: PushPanel,
    settings: SettingsPanel,
} as const;

const active = ref<TabKey>("devices");
const currentPanel = computed(() => PANELS[active.value]);

const { status, refresh } = useApiKey();

onMounted(async () => {
    try {
        await refresh();
    } catch {
        // 读取失败一律按「未配置」处理，由下方引导条统一提示
    }
    if (!status.value.configured) {
        active.value = "settings";
    }
});
</script>
