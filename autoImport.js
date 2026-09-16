/**
 * unplugin-auto-import 配置
 *
 * 作用：Vue / Vue Router / Pinia 的基础 API 无需手写 import，直接调用即可。
 *   ref / computed / watch / onMounted  →  "vue"
 *   useRouter / useRoute / onBeforeEnter →  "vue-router"
 *   defineStore / storeToRefs / acceptHMRUpdate →  "pinia"
 *
 * 类型声明产物 src/auto-imports.d.ts 随仓库提交：`yarn typecheck`（vue-tsc）不经过 vite，
 * 若该文件缺失会整片报「找不到名称 ref」。改动本文件后重跑一次 `yarn dev` 或 `yarn build` 即可重新生成。
 */
import AutoImport from "unplugin-auto-import/vite";

export default AutoImport({
    imports: ["vue", "vue-router", "pinia"],

    // 类型声明输出位置（tsconfig 的 include 已覆盖 src/**/*.d.ts）
    dts: "src/auto-imports.d.ts",

    // 仅处理源码，避免扫描 node_modules 拖慢冷启动
    include: [/\.[jt]sx?$/, /\.vue$/, /\.vue\?vue/],

    // 生成物不参与 lint/格式化，避免与 prettier 互相拉扯
    eslintrc: { enabled: false },
});
