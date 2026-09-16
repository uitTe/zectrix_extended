/**
 * 极趣云 API 的前端封装层。
 *
 * 这里是前端**唯一**接触云平台的地方 —— 所有请求都通过 `invoke` 交给 Rust 后端，
 * API Key 只存在于 Rust 进程，前端既拿不到也不需要它。
 */
import { invoke } from "@tauri-apps/api/core";

import type { Device, ImageUpload, KeyStatus, PushImageResult, Todo, TodoDraft } from "../types/zectrix";

/** Rust 侧 `ApiError` 序列化后的形状 */
interface RawApiError {
    kind?: string;
    message?: string;
}

export class ZectrixApiError extends Error {
    /** 机器可读分类：missing_api_key / keyring / network / api / decode / invalid */
    readonly kind: string;

    constructor(kind: string, message: string) {
        super(message);
        this.name = "ZectrixApiError";
        this.kind = kind;
    }

    /** 是否属于「Key 没配好」，用于引导用户去设置页 */
    get isKeyIssue(): boolean {
        return this.kind === "missing_api_key" || this.kind === "api";
    }
}

function normalizeError(raw: unknown): ZectrixApiError {
    if (raw instanceof ZectrixApiError) return raw;
    if (raw && typeof raw === "object") {
        const { kind, message } = raw as RawApiError;
        if (typeof message === "string" && message.trim()) {
            return new ZectrixApiError(kind ?? "unknown", message);
        }
    }
    return new ZectrixApiError("unknown", String(raw));
}

async function call<T>(command: string, args: Record<string, unknown> = {}): Promise<T> {
    try {
        return await invoke<T>(command, args);
    } catch (raw) {
        throw normalizeError(raw);
    }
}

// ==================== API Key ====================

export const apiKeyStatus = () => call<KeyStatus>("api_key_status");
export const saveApiKey = (apiKey: string) => call<KeyStatus>("save_api_key", { apiKey });
export const clearApiKey = () => call<KeyStatus>("clear_api_key");

/** 「测试连接」：真打一次 /devices，成功即说明 Key 有效 */
export const verifyApiKey = () => call<Device[]>("verify_api_key");

// ==================== 设备 ====================

export const listDevices = () => call<Device[]>("list_devices");

// ==================== 待办 ====================

export const listTodos = (options: { status?: number; deviceId?: string } = {}) =>
    call<Todo[]>("list_todos", {
        status: options.status ?? null,
        deviceId: options.deviceId ?? null,
    });

export const createTodo = (draft: TodoDraft) => call<void>("create_todo", { draft });
export const updateTodo = (id: number, draft: TodoDraft) => call<void>("update_todo", { id, draft });
export const toggleTodo = (id: number) => call<void>("toggle_todo", { id });
export const deleteTodo = (id: number) => call<void>("delete_todo", { id });

// ==================== 显示推送 ====================

export const pushText = (params: { deviceId: string; text: string; fontSize?: number; pageId?: string }) =>
    call<void>("push_text", {
        deviceId: params.deviceId,
        text: params.text,
        fontSize: params.fontSize ?? null,
        pageId: params.pageId ?? null,
    });

export const pushStructuredText = (params: { deviceId: string; title: string; body: string; pageId?: string }) =>
    call<void>("push_structured_text", {
        deviceId: params.deviceId,
        title: params.title,
        body: params.body,
        pageId: params.pageId ?? null,
    });

export const pushImage = (params: { deviceId: string; images: ImageUpload[]; dither?: boolean; pageId?: string }) =>
    call<PushImageResult | null>("push_image", {
        deviceId: params.deviceId,
        images: params.images,
        dither: params.dither ?? true,
        pageId: params.pageId ?? null,
    });

/** 不传 pageId 会清空该设备的全部页面 */
export const clearPages = (params: { deviceId: string; pageId?: string }) =>
    call<void>("clear_pages", {
        deviceId: params.deviceId,
        pageId: params.pageId ?? null,
    });

// ==================== 文件辅助 ====================

/** 分块转 base64，避免一次性展开超长参数列表 */
function arrayBufferToBase64(buffer: ArrayBuffer): string {
    const bytes = new Uint8Array(buffer);
    const CHUNK_SIZE = 0x8000;
    let binary = "";
    for (let offset = 0; offset < bytes.length; offset += CHUNK_SIZE) {
        binary += String.fromCharCode(...bytes.subarray(offset, offset + CHUNK_SIZE));
    }
    return btoa(binary);
}

/** 把浏览器 File 对象转成后端需要的入参 */
export async function fileToImageUpload(file: File): Promise<ImageUpload> {
    const buffer = await file.arrayBuffer();
    return {
        name: file.name,
        mime: file.type || "image/png",
        dataBase64: arrayBufferToBase64(buffer),
    };
}
