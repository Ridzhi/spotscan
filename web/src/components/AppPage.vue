<script setup lang="ts">
import ContentWrap from "@/components/ContentWrap.vue";
</script>

<template>
  <div class="app-page">
    <ContentWrap>
      <h2 v-if="$slots.title" style="margin-bottom: 0">
        <slot name="title" />
      </h2>
      <div v-if="$slots['sub-title']" style="color: var(--van-text-color-2)">
        <slot name="sub-title" />
      </div>
    </ContentWrap>

    <van-divider />

    <slot />
  </div>
</template>

<style scoped>
.app-page {
  width: 100%;
  height: 100%;
  position: relative;
}
</style>
