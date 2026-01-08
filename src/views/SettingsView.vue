<script setup lang="ts">
import { useRouter } from 'vue-router';
import AccountManagement from '@/components/AccountManagement.vue';

const router = useRouter();

function handleClose() {
  router.push('/inbox');
}
</script>

<template>
  <AccountManagement @close="handleClose" />
</template>
