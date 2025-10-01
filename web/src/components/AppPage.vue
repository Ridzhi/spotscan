<script setup lang="ts">
import type {RouteLocationRaw} from "vue-router";
import {useRouter} from "vue-router";
import ContentWrap from "@/components/ContentWrap.vue";
import MainGap from "@/components/MainGap.vue";
interface Props {
  onClose?: RouteLocationRaw | null;
}

const props = defineProps<Props>();
const router = useRouter();

function onClick() {
  if (props.onClose === null) {
    return router.back();
  }

  if (props.onClose) {
    router.push(props.onClose);
  }
}
</script>

<template>
  <div class="app-page">
    <van-icon v-if="props.onClose || props.onClose === null" @click="onClick" class="close-icon" name="cross" color="var(--van-gray-5)" size="22"/>

    <ContentWrap>
      <div class="row x-align-sb y-align-baseline">
        <div>
          <h2 v-if="$slots.title">
            <slot name="title" />
          </h2>
        </div>
        <div>
          <slot name="actions" />
        </div>
      </div>

      <div v-if="$slots['sub-title']" class="text-secondary text-subtitle">
        <slot name="sub-title" />
      </div>
    </ContentWrap>

    <MainGap />

    <slot />
  </div>
</template>

<style scoped>
.app-page {
  width: 100%;
  height: 100%;
  position: relative;

  .close-icon {
    position: absolute;
    top: 20px;
    right: var(--van-padding-md);
  }
}
</style>
<style>
.van-popup .app-page .close-icon {
  top: 0;
}
</style>