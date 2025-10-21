<script setup lang="ts">
import { useIndexStore } from '@/stores';

const store = useIndexStore();
await store.load();
</script>

<template>
  <slot v-bind:theme="store.vanTheme" />
</template>
