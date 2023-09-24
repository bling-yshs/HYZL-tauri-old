<template>
  <a-layout has-sider>
    <a-layout-sider
        :style="{  overflow: 'auto', height: '100vh', position: 'fixed', left: 0, top: 0, bottom: 0 }"
        theme="light"
    >
      <div class="logo"/>
      <a-menu v-model:selectedKeys="selectedKeys" theme="light" mode="inline"
              :style="{padding:'1rem',height: '100vh'}">
        <a-menu-item key="1">
          <RocketTwoTone/>
          <span class="nav-text">一键启动</span>
          <RouterLink to="/start"></RouterLink>
        </a-menu-item>
        <a-menu-item key="2">
          <AppstoreTwoTone/>
          <span class="nav-text">云崽管理</span>
          <RouterLink to="/management"></RouterLink>
        </a-menu-item>
        <a-menu-item key="3">
          <BugTwoTone/>
          <span class="nav-text">问题修复</span>
          <RouterLink to="/issue-fix"></RouterLink>
        </a-menu-item>
        <a-menu-item key="4">
          <SettingTwoTone/>
          <span class="nav-text">设置</span>
          <RouterLink to="/setting"></RouterLink>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
    <a-layout :style="{ marginLeft: '200px' }">
      <a-layout-content :style="{ overflow: 'initial' }">
        <RouterView id="functionPage"/>
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>
<script lang="ts" setup>


import {ref} from 'vue';

import {AppstoreTwoTone, BugTwoTone, RocketTwoTone, SettingTwoTone} from '@ant-design/icons-vue';

const selectedKeys = ref<string[]>(['1']);
</script>
<style scoped>

#components-layout-demo-fixed-sider .logo {
  height: 32px;
  background: rgba(255, 255, 255, 0.2);
  margin: 16px;
}

#functionPage {
  height: 100vh;
}


.ant-layout {
  background-color: #fff;
}

#functionPage .ant-layout {
  margin-left: 0 !important;
}

</style>


