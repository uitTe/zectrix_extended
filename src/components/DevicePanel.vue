<template>
    <section class="panel">
        <header class="panel-head">
            <h2>设备</h2>
            <p>账号下已绑定的 Note4 设备。<code>deviceId</code> 是设备 MAC，后续待办与推送都以它为目标。</p>
        </header>

        <div class="toolbar">
            <button class="btn btn-primary" :disabled="loading" @click="load">
                {{ loading ? "读取中…" : "刷新设备列表" }}
            </button>
            <span v-if="loaded" class="hint">共 {{ devices.length }} 台</span>
        </div>

        <p v-if="error" class="feedback feedback-err">{{ error }}</p>

        <p v-else-if="loaded && devices.length === 0" class="empty">
            账号下还没有设备。请先在手机小程序里完成 Note4 的配网绑定。
        </p>

        <ul v-else class="device-list">
            <li
                v-for="device in devices"
                :key="device.deviceId"
                class="device-item"
                :class="{ selected: device.deviceId === selectedDeviceId }"
            >
                <div class="device-main">
                    <strong>{{ deviceLabel(device) }}</strong>
                    <code class="mono-sub">{{ device.deviceId }}</code>
                    <span v-if="device.board" class="tag">{{ device.board }}</span>
                </div>
                <button
                    class="btn btn-small"
                    :disabled="device.deviceId === selectedDeviceId"
                    @click="select(device.deviceId)"
                >
                    {{ device.deviceId === selectedDeviceId ? "当前设备" : "设为当前" }}
                </button>
            </li>
        </ul>
    </section>
</template>

<script setup lang="ts">
import { ZectrixApiError } from "../api/zectrix";
import { useDevices } from "../composables/useDevices";
import { deviceLabel } from "../types/zectrix";

const { devices, loading, loaded, selectedDeviceId, refresh, select } = useDevices();

const error = ref<string | null>(null);

async function load() {
    error.value = null;
    try {
        await refresh();
    } catch (e) {
        error.value = e instanceof ZectrixApiError ? e.message : String(e);
    }
}

onMounted(load);
</script>
