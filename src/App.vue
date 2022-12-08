<template>
  <div class="m-6">
    <notifications />
  </div>
</template>

<script setup lang="ts">
  import { onMounted } from 'vue';
  import notifications from './components/notifications.vue';

  onMounted(() => {
    document.querySelector('body')?.setAttribute('class', 'bg-gray-200');
});
</script>
