<script setup lang="ts">
import { useBackButton } from '@/composables/useBackButton';
import AsyncWrap from "@/components/AsyncWrap.vue";
import AppInit from "@/components/AppInit.vue";

// @TODO check availability in real env
useBackButton();

</script>

<template>
  <router-view v-slot="{Component, route}">
    <Transition>
      <div :key="route.name">
        <KeepAlive>
          <AsyncWrap>
            <AppInit v-slot="{theme}">
              <van-config-provider :theme="theme">
                <component :is="Component" />
              </van-config-provider>
            </AppInit>
          </AsyncWrap>
        </KeepAlive>
      </div>
    </Transition>
  </router-view>
</template>
