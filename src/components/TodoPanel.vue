<template>
    <section class="panel">
        <header class="panel-head">
            <h2>待办</h2>
            <p>云端待办与设备同源 —— 这里改完，Note4 屏幕上也会同步。</p>
        </header>

        <div class="card">
            <div class="card-head">
                <h3>{{ editingId === null ? "新建待办" : `编辑待办 #${editingId}` }}</h3>
                <button v-if="editingId !== null" class="btn btn-small" @click="resetForm">取消编辑</button>
            </div>

            <div class="form-grid">
                <label class="span-2">
                    <span>标题</span>
                    <input v-model="form.title" placeholder="例如：买牛奶" @keyup.enter="submit" />
                </label>
                <label>
                    <span>截止日期</span>
                    <input v-model="form.dueDate" type="date" />
                </label>
                <label>
                    <span>截止时间</span>
                    <input v-model="form.dueTime" type="time" />
                </label>
                <label>
                    <span>优先级</span>
                    <input v-model.number="form.priority" type="number" min="0" max="9" />
                </label>
                <label>
                    <span>关联设备</span>
                    <select v-model="form.deviceId">
                        <option value="">不关联</option>
                        <option v-for="device in devices" :key="device.deviceId" :value="device.deviceId">
                            {{ deviceLabel(device) }}
                        </option>
                    </select>
                </label>
                <label class="span-2">
                    <span>描述</span>
                    <input v-model="form.description" placeholder="可选" />
                </label>
            </div>

            <div class="field-row">
                <button class="btn btn-primary" :disabled="saving" @click="submit">
                    {{ saving ? "提交中…" : editingId === null ? "创建" : "保存修改" }}
                </button>
            </div>
        </div>

        <div class="toolbar">
            <div class="segmented">
                <button
                    v-for="option in STATUS_OPTIONS"
                    :key="option.key"
                    class="seg-btn"
                    :class="{ active: statusFilter === option.key }"
                    @click="setStatusFilter(option.key)"
                >
                    {{ option.label }}
                </button>
            </div>

            <select v-model="deviceFilter" class="inline-select" @change="load">
                <option value="">全部设备</option>
                <option v-for="device in devices" :key="device.deviceId" :value="device.deviceId">
                    {{ deviceLabel(device) }}
                </option>
            </select>

            <button class="btn btn-small" :disabled="loading" @click="load">
                {{ loading ? "读取中…" : "刷新" }}
            </button>
        </div>

        <p v-if="error" class="feedback feedback-err">{{ error }}</p>
        <p v-else-if="notice" class="feedback feedback-ok">{{ notice }}</p>

        <p v-if="loaded && todos.length === 0 && !error" class="empty">没有符合条件的待办。</p>

        <ul v-else class="todo-list">
            <li v-for="todo in todos" :key="todo.id" class="todo-item" :class="{ done: isDone(todo) }">
                <div class="todo-main">
                    <div class="todo-title">
                        <strong>{{ todo.title || "(无标题)" }}</strong>
                        <span v-if="todo.priority" class="tag tag-priority">P{{ todo.priority }}</span>
                        <span v-if="isDone(todo)" class="tag tag-done">已完成</span>
                    </div>
                    <p v-if="todo.description" class="todo-desc">{{ todo.description }}</p>
                    <p class="todo-meta">
                        <span v-if="todo.dueDate">{{ todo.dueDate }} {{ todo.dueTime ?? "" }}</span>
                        <span v-if="todo.deviceName || todo.deviceId">{{ todo.deviceName || todo.deviceId }}</span>
                        <span>#{{ todo.id }}</span>
                    </p>
                </div>
                <div class="todo-actions">
                    <button class="btn btn-small" @click="onToggle(todo)">
                        {{ isDone(todo) ? "取消完成" : "完成" }}
                    </button>
                    <button class="btn btn-small" @click="startEdit(todo)">编辑</button>
                    <button
                        class="btn btn-small"
                        :class="pendingDeleteId === todo.id ? 'btn-danger' : ''"
                        @click="onDelete(todo)"
                    >
                        {{ pendingDeleteId === todo.id ? "确认删除" : "删除" }}
                    </button>
                </div>
            </li>
        </ul>
    </section>
</template>

<script setup lang="ts">
import { createTodo, deleteTodo, listTodos, toggleTodo, updateTodo, ZectrixApiError } from "../api/zectrix";
import { useDevices } from "../composables/useDevices";
import { deviceLabel, type Todo, type TodoDraft } from "../types/zectrix";

const { devices, selectedDeviceId, refresh: refreshDevices } = useDevices();

type StatusFilter = "all" | "open" | "done";

const STATUS_OPTIONS: { key: StatusFilter; label: string }[] = [
    { key: "open", label: "待完成" },
    { key: "done", label: "已完成" },
    { key: "all", label: "全部" },
];

const statusFilter = ref<StatusFilter>("open");
const deviceFilter = ref("");
const todos = ref<Todo[]>([]);
const loading = ref(false);
const loaded = ref(false);
const saving = ref(false);
const error = ref<string | null>(null);
const notice = ref<string | null>(null);
const editingId = ref<number | null>(null);
const pendingDeleteId = ref<number | null>(null);

const form = reactive({
    title: "",
    description: "",
    dueDate: "",
    dueTime: "",
    priority: 0,
    deviceId: "",
});

function describe(error: unknown): string {
    return error instanceof ZectrixApiError ? error.message : String(error);
}

async function load() {
    loading.value = true;
    error.value = null;
    try {
        todos.value = await listTodos({
            status: statusFilter.value === "open" ? 0 : statusFilter.value === "done" ? 1 : undefined,
            deviceId: deviceFilter.value || undefined,
        });
        loaded.value = true;
    } catch (e) {
        error.value = describe(e);
    } finally {
        loading.value = false;
    }
}

function resetForm() {
    editingId.value = null;
    form.title = "";
    form.description = "";
    form.dueDate = "";
    form.dueTime = "";
    form.priority = 0;
    form.deviceId = selectedDeviceId.value;
}

function startEdit(todo: Todo) {
    editingId.value = todo.id;
    form.title = todo.title ?? "";
    form.description = todo.description ?? "";
    form.dueDate = todo.dueDate ?? "";
    form.dueTime = todo.dueTime ?? "";
    form.priority = todo.priority ?? 0;
    form.deviceId = todo.deviceId ?? "";
    error.value = null;
    notice.value = null;
}

function buildDraft(): TodoDraft {
    const draft: TodoDraft = { title: form.title.trim() };
    if (form.description.trim()) draft.description = form.description.trim();
    if (form.dueDate) draft.dueDate = form.dueDate;
    if (form.dueTime) draft.dueTime = form.dueTime;
    if (form.priority) draft.priority = Number(form.priority);
    if (form.deviceId) draft.deviceId = form.deviceId;
    return draft;
}

async function submit() {
    if (!form.title.trim()) {
        error.value = "标题不能为空";
        return;
    }
    saving.value = true;
    error.value = null;
    notice.value = null;
    try {
        if (editingId.value === null) {
            await createTodo(buildDraft());
            notice.value = "待办已创建";
        } else {
            await updateTodo(editingId.value, buildDraft());
            notice.value = "待办已更新";
        }
        resetForm();
        await load();
    } catch (e) {
        error.value = describe(e);
    } finally {
        saving.value = false;
    }
}

async function onToggle(todo: Todo) {
    error.value = null;
    notice.value = null;
    try {
        await toggleTodo(todo.id);
        await load();
    } catch (e) {
        error.value = describe(e);
    }
}

async function onDelete(todo: Todo) {
    if (pendingDeleteId.value !== todo.id) {
        pendingDeleteId.value = todo.id;
        return;
    }
    pendingDeleteId.value = null;
    error.value = null;
    notice.value = null;
    try {
        await deleteTodo(todo.id);
        notice.value = "待办已删除";
        if (editingId.value === todo.id) resetForm();
        await load();
    } catch (e) {
        error.value = describe(e);
    }
}

function isDone(todo: Todo): boolean {
    return todo.completed === true || todo.status === 1;
}

function setStatusFilter(key: StatusFilter) {
    statusFilter.value = key;
    void load();
}

onMounted(async () => {
    try {
        await refreshDevices();
    } catch {
        // 设备列表拉取失败不阻塞待办；待办本身会给出错误提示
    }
    resetForm();
    await load();
});
</script>
