<script setup lang="ts">

import {ref} from "vue";

const startLoadingState = ref<boolean>(false);

const startLoadingFn = () => {
  startLoadingState.value = true
}

</script>

<template>
  <div>
    <div id="main">
      <a-button type="default" :loading="startLoadingState" @click="startLoadingFn">一键启动</a-button>
      
    </div>
  </div>
</template>

<style scoped>
#main {
  padding: 1rem;
}
</style>