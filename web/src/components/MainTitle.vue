<template>
  <h1>
    <slot></slot>
  </h1>
  <div
    v-if="$slots.sub"
    class="text-secondary text-subtitle"
  >
    <slot name="sub"></slot>
  </div>
</template>
