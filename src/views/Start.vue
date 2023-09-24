<script setup lang="ts">
import {message} from "ant-design-vue";
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from "vue";
import {CaretRightOutlined} from "@ant-design/icons-vue";
import { h } from 'vue';
import {StringDataResponse} from "../entity/String/string_data_response.ts";

const startYunzaiLoadingState = ref<boolean>(false);

const [messageApi, contextHolder] = message.useMessage();

let num = ref(0);
const startYunzai = () => {
  startYunzaiLoadingState.value = true
  invoke('start_yunzai').then((response) => {
    let res = JSON.parse(response as string) as StringDataResponse;
    console.log(res)
    startYunzaiLoadingState.value = false
    if (res.code === 200) messageApi.success(res.message);
    else messageApi.error(res.message);
  })
};
</script>

<template>
  <context-holder/>
  <div id="main">
    <a-space direction="vertical">
      <a-space direction="horizontal">
        <a-button type="primary"
                  :loading="startYunzaiLoadingState"
                  :icon="h(CaretRightOutlined)"
                  @click="startYunzai">
          一键启动
        </a-button>
        <a-button @click="startYunzai">
          {{ num }}
        </a-button>
      </a-space>
    </a-space>
  </div>
</template>

<style scoped>
#main {
  padding: 1rem;
}
</style>