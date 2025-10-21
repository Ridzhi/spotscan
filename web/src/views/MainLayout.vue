<script setup lang="ts">
import { ref } from 'vue';

const active = ref(0);
</script>

<template>
  <div style="padding-bottom: calc(var(--van-tabbar-height) + var(--van-padding-md))">
    <router-view />
  </div>
  <div class="van-clearfix"></div>
  <van-tabbar
    id="tabbar"
    v-model="active"
    route
    safe-area-inset-bottom
  >
    <van-tabbar-item
      replace
      :to="{ name: 'slots' }"
      icon="calendar_view_month"
      icon-prefix="gmi"
    >
      Слоты
    </van-tabbar-item>
    <van-tabbar-item
      replace
      :to="{ name: 'settings' }"
      icon="settings"
      icon-prefix="gmi"
    >
      Настройки
    </van-tabbar-item>
  </van-tabbar>
</template>
