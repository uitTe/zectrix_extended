import App from "./App.vue";
import "./styles.scss";

// createApp 等基础 API 由 autoImport.js 自动导入，无需手写 import
// 未注册 Pinia：当前共享状态只有 2 个 composable 单例，用不到 store；autoImport.js 仍保留 pinia 预设，
// 将来需要时直接 defineStore 即可，无需改构建配置（判断依据见 .workbuddy/memory/MEMORY.md）
createApp(App).mount("#app");
