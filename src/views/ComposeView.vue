<script setup lang="ts">
import { useRouter } from 'vue-router';
import EmailComposer from '@/components/EmailComposer.vue';

const router = useRouter();

function handleClose() {
  router.push('/inbox');
}

function handleSent() {
  router.push('/inbox');
}
</script>

<template>
  <EmailComposer :account-id="1" @close="handleClose" @sent="handleSent" />
</template>
