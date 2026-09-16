import { apiKeyStatus, clearApiKey, saveApiKey, verifyApiKey } from "../api/zectrix";
import type { Device, KeyStatus } from "../types/zectrix";

/** 模块级单例：多个组件共享同一份 Key 状态 */
const status = ref<KeyStatus>({ configured: false, masked: null });
const loaded = ref(false);

export function useApiKey() {
    async function refresh(): Promise<KeyStatus> {
        try {
            status.value = await apiKeyStatus();
        } finally {
            loaded.value = true;
        }
        return status.value;
    }

    async function save(apiKey: string): Promise<KeyStatus> {
        status.value = await saveApiKey(apiKey);
        return status.value;
    }

    async function clear(): Promise<KeyStatus> {
        status.value = await clearApiKey();
        return status.value;
    }

    /** 真打一次 /devices，用来验证 Key 是否有效 */
    async function verify(): Promise<Device[]> {
        return verifyApiKey();
    }

    return { status, loaded, refresh, save, clear, verify };
}
