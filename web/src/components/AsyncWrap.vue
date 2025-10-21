<template>
  <Suspense>
    <template #default>
      <div>
        <slot></slot>
      </div>
    </template>
    <template #fallback>
      <div style="display: flex; justify-content: center; align-items: center; height: 100vh">
        <van-loading
          type="spinner"
          size="60"
        />
      </div>
    </template>
  </Suspense>
</template>
