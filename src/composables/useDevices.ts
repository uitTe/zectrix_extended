import { listDevices } from "../api/zectrix";
import type { Device } from "../types/zectrix";

/** 模块级单例：设备列表在「设备 / 待办 / 推送」三个面板间共享 */
const devices = ref<Device[]>([]);
const loading = ref(false);
const loaded = ref(false);
const selectedDeviceId = ref("");

const selectedDevice = computed(
    () => devices.value.find((device) => device.deviceId === selectedDeviceId.value) ?? null,
);

export function useDevices() {
    async function refresh(): Promise<Device[]> {
        loading.value = true;
        try {
            const list = await listDevices();
            devices.value = list;
            loaded.value = true;
            // 当前选中的设备若已不在列表中（被解绑等），自动回落到第一台
            if (!list.some((device) => device.deviceId === selectedDeviceId.value)) {
                selectedDeviceId.value = list[0]?.deviceId ?? "";
            }
            return list;
        } finally {
            loading.value = false;
        }
    }

    function select(deviceId: string) {
        selectedDeviceId.value = deviceId;
    }

    return { devices, loading, loaded, selectedDeviceId, selectedDevice, refresh, select };
}
