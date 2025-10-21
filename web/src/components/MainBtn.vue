<template>
  <van-button
    class="main-btn"
    v-bind="$attrs"
    type="primary"
    size="large"
  >
    <slot></slot>
  </van-button>
</template>

<style scoped>
.main-btn {
  font-size: 16px;
  font-weight: 600;
  border-radius: 16px;
}
</style>
