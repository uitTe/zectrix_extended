/**
 * 极趣云开放 API 的数据类型，与 Rust 侧 `src-tauri/src/api/models.rs` 一一对应。
 * 字段名保持云端约定的 camelCase。
 */

/** 设备。`deviceId` 即设备 MAC 地址，所有设备级接口都用它。 */
export interface Device {
    deviceId: string;
    alias?: string | null;
    board?: string | null;
}

/** 待办事项 */
export interface Todo {
    id: number;
    title: string;
    description?: string | null;
    /** YYYY-MM-DD */
    dueDate?: string | null;
    /** HH:mm */
    dueTime?: string | null;
    repeatType?: string | null;
    /** 0 = 待完成，1 = 已完成 */
    status?: number | null;
    priority?: number | null;
    completed?: boolean | null;
    deviceId?: string | null;
    deviceName?: string | null;
    createDate?: string | null;
    /** 注意：云端字段名是 `updateDate`，不是 `updatedAt` */
    updateDate?: number | null;
}

/** 创建 / 更新待办的提交体。未填的字段不会发送。 */
export interface TodoDraft {
    title?: string;
    description?: string;
    dueDate?: string;
    dueTime?: string;
    repeatType?: string;
    priority?: number;
    deviceId?: string;
}

/** 图片推送入参：文件读成 base64 后交给 Rust 解码 */
export interface ImageUpload {
    name: string;
    mime: string;
    dataBase64: string;
}

/** 图片推送结果 */
export interface PushImageResult {
    totalPages?: number | null;
    pushedPages?: number | null;
    pageId?: string | null;
}

/** API Key 配置状态。只回传掩码，完整 Key 不离开 Rust 进程。 */
export interface KeyStatus {
    configured: boolean;
    masked?: string | null;
}

/** 界面上可选的页面编号（云端支持 1–5） */
export const PAGE_IDS = ["1", "2", "3", "4", "5"] as const;

/** 设备展示名：优先别名，缺省回退 MAC */
export function deviceLabel(device: Device | undefined | null): string {
    if (!device) return "未选择设备";
    const alias = device.alias?.trim();
    return alias ? alias : device.deviceId;
}
